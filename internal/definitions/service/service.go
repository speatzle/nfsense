package service

type Service struct {
	DHCPv4 []DHCPv4 `json:"dhcp_v4" validate:"required,dive"`
	DHCPv6 []DHCPv6 `json:"dhcp_v6" validate:"required,dive"`
}
