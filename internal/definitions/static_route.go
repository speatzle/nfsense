package definitions

import (
	"net/netip"
)

type StaticRoute struct {
	Name        string     `json:"name,omitempty"`
	Interface   string     `json:"interface,omitempty"`
	Gateway     netip.Addr `json:"gateway,omitempty"`
	Destination IPNet      `json:"destination,omitempty"`
	Metric      uint       `json:"metric,omitempty"`
}
