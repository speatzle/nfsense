package util

import (
	"net"
)

func BroadcastAddr(n net.IPNet) net.IP {
	var broadcast net.IP
	var length int
	if n.IP.To4() != nil {
		broadcast = net.ParseIP("0.0.0.0").To4()
		length = 4
	} else {
		broadcast = net.ParseIP("::")
		length = 16

	}
	for i := 0; i < length; i++ {
		broadcast[i] = n.IP[i] | ^n.Mask[i]
	}
	return broadcast
}
