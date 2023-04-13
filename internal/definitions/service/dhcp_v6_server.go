package service

import "time"

type DHCPv6Server struct {
	Interface        string
	Pool             []string
	DefaultLeaseTime time.Duration
	MaxLeaseTime     time.Duration

	GatewayMode   Mode
	Gateway       *string
	DNSServerMode Mode
	DNSServer     *[]string
	NTPServerMode Mode
	NTPServer     *[]string

	Reservations []Reservation
}
