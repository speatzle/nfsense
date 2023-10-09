package network

import (
	"encoding/json"
	"net/netip"
)

type Interface struct {
	Alias          string                  `json:"alias,omitempty"`
	Type           InterfaceType           `json:"type"`
	AddressingMode InterfaceAddressingMode `json:"addressing_mode"`
	Address        *netip.Prefix           `json:"address,omitempty"`
	HardwareDevice *string                 `json:"hardware_device,omitempty"`
	// TODO fix Validator for int pointers with min=0,max=4094
	VlanID        *uint     `json:"vlan_id,omitempty"`
	VlanParent    *string   `json:"vlan_parent,omitempty"`
	BondMembers   *[]string `json:"bond_members,omitempty"`
	BridgeMembers *[]string `json:"bridge_members,omitempty"`
	Comment       string    `json:"comment,omitempty"`
}

type InterfaceType int

const (
	Hardware InterfaceType = iota
	Vlan
	Bond
	Bridge
)

func (t InterfaceType) String() string {
	return [...]string{"hardware", "vlan", "bond", "bridge"}[t]
}

func (t *InterfaceType) FromString(input string) InterfaceType {
	return map[string]InterfaceType{
		"hardware": Hardware,
		"vlan":     Vlan,
		"bond":     Bond,
		"bridge":   Bridge,
	}[input]
}

func (t InterfaceType) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *InterfaceType) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}

type InterfaceAddressingMode int

const (
	None InterfaceAddressingMode = iota
	Static
	Dhcp
)

func (t InterfaceAddressingMode) String() string {
	return [...]string{"none", "static", "dhcp"}[t]
}

func (t *InterfaceAddressingMode) FromString(input string) InterfaceAddressingMode {
	return map[string]InterfaceAddressingMode{
		"none":   None,
		"static": Static,
		"dhcp":   Dhcp,
	}[input]
}

func (t InterfaceAddressingMode) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *InterfaceAddressingMode) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
