package network

import (
	"net/netip"

	"nfsense.net/nfsense/internal/definitions/common"
)

type StaticRoute struct {
	Name        string       `json:"name,omitempty"`
	Interface   string       `json:"interface,omitempty"`
	Gateway     netip.Addr   `json:"gateway,omitempty"`
	Destination common.IPNet `json:"destination,omitempty"`
	Metric      uint         `json:"metric,omitempty"`
}
