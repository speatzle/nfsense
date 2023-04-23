package network

import (
	"net/netip"
)

type StaticRoute struct {
	Name        string       `json:"name,omitempty"`
	Interface   string       `json:"interface,omitempty"`
	Gateway     netip.Addr   `json:"gateway,omitempty"`
	Destination netip.Prefix `json:"destination,omitempty"`
	Metric      uint         `json:"metric,omitempty"`
}
