package definitions

import "encoding/json"

type SourceNATRule struct {
	Rule
	Type    SnatType `json:"type" validate:"min=0,max=1"`
	Address string   `json:"address,omitempty"`
	Service string   `json:"service,omitempty"`
}

type SnatType int

const (
	Snat SnatType = iota
	Masquerade
)

func (t SnatType) String() string {
	return [...]string{"snat", "masquerade"}[t]
}

func (t *SnatType) FromString(input string) SnatType {
	return map[string]SnatType{
		"snat":       Snat,
		"masquerade": Masquerade,
	}[input]
}

func (t SnatType) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *SnatType) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
