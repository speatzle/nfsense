package common

import (
	"encoding/json"
	"net"
)

type IPNet struct {
	net.IPNet
}

// MarshalJSON for IPNet
func (i IPNet) MarshalJSON() ([]byte, error) {
	return json.Marshal(i.String())
}

// UnmarshalJSON for IPNet
func (i *IPNet) UnmarshalJSON(b []byte) error {
	var s string
	if err := json.Unmarshal(b, &s); err != nil {
		return err
	}

	_, ipnet, err := net.ParseCIDR(s)
	if err != nil {
		return err
	}
	i.IPNet = *ipnet
	return nil
}
