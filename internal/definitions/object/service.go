package object

import (
	"encoding/json"
	"fmt"
)

type Service struct {
	Type       ServiceType `json:"type" validate:"min=0,max=3"`
	Comment    string      `json:"comment,omitempty"`
	SPortStart *uint32     `json:"sport_start,omitempty" validate:"excluded_unless=Type 0|excluded_unless=Type 1"`
	SPortEnd   *uint32     `json:"sport_end,omitempty"`
	DPortStart *uint32     `json:"dport_start,omitempty" validate:"excluded_unless=Type 0|excluded_unless=Type 1"`
	DPortEnd   *uint32     `json:"dport_end,omitempty"`
	ICMPCode   *uint32     `json:"icmp_code,omitempty" validate:"excluded_unless=Type 2"`
	Children   *[]string   `json:"children,omitempty"`
}

func (s Service) GetSPort() string {
	if s.SPortStart == nil || *s.SPortStart == 0 {
		return ""
	} else if s.SPortEnd == nil || *s.SPortEnd == 0 {
		return fmt.Sprintf("%d", *s.SPortStart)
	}
	return fmt.Sprintf("%d - %d", *s.SPortStart, *s.SPortEnd)
}

func (s Service) GetDPort() string {
	if s.DPortStart == nil || *s.DPortStart == 0 {
		return ""
	} else if s.DPortEnd == nil || *s.DPortEnd == 0 {
		return fmt.Sprintf("%d", *s.DPortStart)
	}
	return fmt.Sprintf("%d - %d", *s.DPortStart, *s.DPortEnd)
}

type ServiceType int

const (
	TCP ServiceType = iota
	UDP
	ICMP
	ServiceGroup
)

func (t ServiceType) String() string {
	return [...]string{"tcp", "udp", "icmp", "group"}[t]
}

func (t *ServiceType) FromString(input string) ServiceType {
	return map[string]ServiceType{
		"tcp":   TCP,
		"udp":   UDP,
		"icmp":  ICMP,
		"group": ServiceGroup,
	}[input]
}

func (t ServiceType) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *ServiceType) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
