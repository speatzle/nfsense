#!/bin/bash
echo "Installing nfsense dev env..."
if [ "$(id -u)" -ne 0 ]; then echo "Please run as root." >&2; exit 1; fi
cd /root


echo "Installing Required Packages from Repos"
apt update
DEBIAN_FRONTEND=noninteractive apt install tcpdump kea unbound chrony wireguard-tools tar ulogd2 ulogd2-json -y

echo "Installing Required Dev Packages from Repos"
DEBIAN_FRONTEND=noninteractive apt install curl git nodejs gcc pkg-config libssl-dev -y

echo "Installing pnpm"
curl -fsSL https://get.pnpm.io/install.sh | sh -
source /root/.bashrc

echo "Installing rust"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

echo "Clone nfsense Repo"
if [ ! -d "/opt/nfsense" ]; then
    cd /opt
    git clone "https://github.com/speatzle/nfsense.git"
else
    echo "nfsense folder already exists"
fi
cd /opt/nfsense

echo "Generate SSL Certificates"
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -subj "/C=XX/ST=StateName/L=CityName/O=CompanyName/OU=CompanySectionName/CN=CommonNameOrHostname"

echo "Build nfSense"
cargo build

echo "Install Client deps"
cd client
pnpm install
cd ..

echo "Setup nfSense systemd services"
cat <<EOT >> /etc/systemd/system/nfsense.service
[Unit]
Description=nfsense api
ConditionPathExists=/opt/nfsense
After=network.target

[Service]
Type=simple
WorkingDirectory=/opt/nfsense
ExecStart=/opt/nfsense/target/debug/nfsense
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

mkdir /etc/nfsense

touch /etc/chrony/conf.d/chrony-nfsense.conf

unbound-control-setup

echo "reload systemd"
systemctl daemon-reload

echo "Setup nftables"
echo '#!/usr/sbin/nft -f\n' > /etc/nftables.conf
echo 'include "/etc/nfsense/nfsense-nftables.conf"' >> /etc/nftables.conf
touch /etc/nfsense/nfsense-nftables.conf

echo "Disable ifupdown"
mv /etc/network/interfaces /etc/network/interfaces.old

echo "Allow Routing"
echo 'net.ipv4.ip_forward=1' > /etc/sysctl.d/99-nfsense.conf
systemctl restart systemd-sysctl

echo "Generate nfsense Config"
./target/debug/nfsense generate-default

echo "Enable nftables"
systemctl enable nftables

echo "Enable systemd-networkd"
systemctl enable systemd-networkd

echo "Starting nfsense"
systemctl enable --now nfsense
systemctl enable --now nfsense-client

echo "Done"
echo "Note: you need to login to the web interface at https://<your-ip>:8080 with the user admin and password nfsense."
echo "Configure the Network interfaces, your default route and apply the changes."
echo "Otherwise there will be no network connectivity after reboot."
