package service

import (
	"net/netip"

	"nfsense.net/nfsense/internal/definitions/common"
)

type Reservation struct {
	HardwareAddress common.HardwareAddress
	IPAddress       netip.Addr
}
