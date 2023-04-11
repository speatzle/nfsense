package definitions

import (
	"encoding/json"
	"net/netip"

	"go4.org/netipx"
)

type Address struct {
	Type           AddressType     `json:"type" validate:"min=0,max=3"`
	Comment        string          `json:"comment,omitempty"`
	Host           *netip.Addr     `json:"host,omitempty" validate:"excluded_unless=Type 0"`
	Range          *netipx.IPRange `json:"range,omitempty" validate:"excluded_unless=Type 1"`
	NetworkAddress *IPNet          `json:"network,omitempty" validate:"excluded_unless=Type 2"`
	Children       *[]string       `json:"children,omitempty"`
}

type AddressType int

const (
	Host AddressType = iota
	Range
	NetworkAddress
	AddressGroup
)

func (t AddressType) String() string {
	return [...]string{"host", "range", "network", "group"}[t]
}

func (t *AddressType) FromString(input string) AddressType {
	return map[string]AddressType{
		"host":    Host,
		"range":   Range,
		"network": NetworkAddress,
		"group":   AddressGroup,
	}[input]
}

func (t AddressType) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *AddressType) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
