package service

import (
	"time"
)

type DHCPv4Server struct {
	Interface        string        `json:"interface"`
	Pool             []string      `json:"pool"`
	DefaultLeaseTime time.Duration `json:"default_lease_time"`
	MaxLeaseTime     time.Duration `json:"max_lease_time"`

	GatewayMode   Mode      `json:"gateway_mode"`
	Gateway       *string   `json:"gateway,omitempty"`
	DNSServerMode Mode      `json:"dns_server_mode"`
	DNSServers    *[]string `json:"dns_servers,omitempty"`
	NTPServerMode Mode      `json:"ntp_server_mode"`
	NTPServers    *[]string `json:"ntp_servers,omitempty"`

	Reservations []Reservation `json:"reservations,omitempty"`

	Comment string `json:"comment,omitempty"`
}
