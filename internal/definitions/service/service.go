package service

type Service struct {
	DHCPv4Servers []DHCPv4Server `json:"dhcp_v4_servers" validate:"required,dive"`
	DHCPv6Servers []DHCPv6Server `json:"dhcp_v6_servers" validate:"required,dive"`
	DNSServers    []DNSServer    `json:"dns_servers" validate:"required,dive"`
}
