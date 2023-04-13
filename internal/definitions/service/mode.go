package service

import "encoding/json"

type Mode int

const (
	None Mode = iota
	Interface
	Specify
)

func (t Mode) String() string {
	return [...]string{"none", "interface", "specify"}[t]
}

func (t *Mode) FromString(input string) Mode {
	return map[string]Mode{
		"none":      None,
		"interface": Interface,
		"specify":   Specify,
	}[input]
}

func (t Mode) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *Mode) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
