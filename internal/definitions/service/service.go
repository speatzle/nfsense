package service

type Service struct {
	DHCPv4Servers []DHCPv4Server `json:"dhcp_v4_servers"`
	DHCPv6Servers []DHCPv6Server `json:"dhcp_v6_servers"`
	DNSServers    []DNSServer    `json:"dns_servers"`
	NTPServers    []NTPServer    `json:"ntp_servers"`
}
