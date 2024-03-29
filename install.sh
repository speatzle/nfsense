#!/bin/bash
echo "Installing nfsense dev env..."
if [ "$(id -u)" -ne 0 ]; then echo "Please run as root." >&2; exit 1; fi
cd /root

echo "Disable firewalld"
systemctl stop firewalld

echo "Disable SE Linux"
setenforce 0
sed -i s/^SELINUX=.*$/SELINUX=permissive/ /etc/selinux/config

echo "Allow Routing"
echo 'net.ipv4.ip_forward=1' >> /etc/sysctl.conf
sysctl -p

echo "Setup Repos"
dnf install epel-release -y
/usr/bin/crb enable
dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo

echo "Installing Required Packages from Repos"
dnf install wget git systemd-networkd gh tcpdump nodejs dhcp-server unbound wireguard-tools tar -y

echo "Installing pnpm"
curl -fsSL https://get.pnpm.io/install.sh | sh -
source /root/.bashrc

echo "Installing go"
wget "https://go.dev/dl/go1.21.2.linux-amd64.tar.gz"
$ rm -rf /usr/local/go && tar -C /usr/local -xzf go1.21.2.linux-amd64.tar.gz
cat <<EOT >> /etc/profile
export PATH=$PATH:/usr/local/go/bin
EOT
source /etc/profile

echo "Installing code server"
curl -fsSL https://code-server.dev/install.sh | sh
mkdir -p /root/.config/code-server
cat <<EOT >> /root/.config/code-server/config.yaml
bind-addr: 0.0.0.0:8081
auth: password
password: nfsense
cert: /root/cert.pem
cert-key: /root/key.pem
EOT
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -subj "/C=XX/ST=StateName/L=CityName/O=CompanyName/OU=CompanySectionName/CN=CommonNameOrHostname"
systemctl enable --now code-server@root

echo "Clone nfsense Repo"
cd /opt
git clone "https://github.com/speatzle/nfsense.git"
cd nfsense

echo "build nfsense"
go build

echo "install client deps"
cd client
pnpm install
cd ..

echo "setup nfsense systemd services"
cat <<EOT >> /etc/systemd/system/nfsense.service
[Unit]
Description=nfsense api
ConditionPathExists=/opt/nfsense
After=network.target

[Service]
Type=simple
WorkingDirectory=/opt/nfsense
ExecStart=/opt/nfsense/nfsense
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
EOT

cat <<EOT >> /etc/systemd/system/nfsense-client.service
[Unit]
Description=nfsense client
ConditionPathExists=/opt/nfsense/client
After=network.target

[Service]
Type=simple
WorkingDirectory=/opt/nfsense/client
ExecStart=/root/.local/share/pnpm/pnpm run dev --host 0.0.0.0
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
EOT

echo "reload systemd"
systemctl daemon-reload

echo "Setup nftables"
echo 'include "/etc/nftables/nfsense.conf"' >> /etc/sysconfig/nftables.conf
touch /etc/nftables/nfsense.conf

echo "Setup nfsense Config"
./nfsense setup

# Enable & Disable Network Services
systemctl enable nftables
systemctl enable systemd-networkd
systemctl disable NetworkManager
systemctl stop NetworkManager

echo "Apply nfsense Config"
./nfsense apply


echo "Starting nfsense"
systemctl enable nfsense
systemctl start nfsense
systemctl enable nfsense-client
systemctl start nfsense-client

echo "Done"
