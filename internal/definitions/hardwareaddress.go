package definitions

import (
	"encoding/json"
	"net"
)

type HardwareAddress struct {
	net.HardwareAddr
}

// MarshalJSON for IPCIDR
func (i HardwareAddress) MarshalJSON() ([]byte, error) {
	return json.Marshal(i.String())
}

// UnmarshalJSON for IPCIDR
func (i *HardwareAddress) UnmarshalJSON(b []byte) error {
	var s string
	if err := json.Unmarshal(b, &s); err != nil {
		return err
	}

	mac, err := net.ParseMAC(s)
	if err != nil {
		return err
	}
	i.HardwareAddr = mac
	return nil
}
