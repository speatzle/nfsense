package common

import (
	"encoding/json"
	"net"
)

// IPCIDR is IP Address with the mask in CIDR format
type IPCIDR struct {
	net.IPNet
}

// MarshalJSON for IPCIDR
func (i IPCIDR) MarshalJSON() ([]byte, error) {
	return json.Marshal(i.String())
}

// UnmarshalJSON for IPCIDR
func (i *IPCIDR) UnmarshalJSON(b []byte) error {
	var s string
	if err := json.Unmarshal(b, &s); err != nil {
		return err
	}

	ip, ipnet, err := net.ParseCIDR(s)
	if err != nil {
		return err
	}
	i.IPNet = *ipnet
	i.IPNet.IP = ip
	return nil
}
