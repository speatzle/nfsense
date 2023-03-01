package definitions

import "encoding/json"

type Service struct {
	Type       ServiceType `json:"type"`
	Comment    string      `json:"comment,omitempty"`
	SPortStart *uint32     `json:"sport_start,omitempty"`
	SPortEnd   *uint32     `json:"sport_end,omitempty"`
	DPortStart *uint32     `json:"dport_start,omitempty"`
	DPortEnd   *uint32     `json:"dport_end,omitempty"`
	ICMPCode   *uint32     `json:"icmp_code,omitempty"`
}

type ServiceType int

const (
	TCP ServiceType = iota
	UDP
	TCPUDP
	ICMP
)

func (t ServiceType) String() string {
	return [...]string{"tcp", "udp", "tcpudp", "icmp"}[t]
}

func (t *ServiceType) FromString(input string) ServiceType {
	return map[string]ServiceType{
		"tcp":    TCP,
		"udp":    UDP,
		"tcpudp": TCPUDP,
		"icmp":   ICMP,
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
