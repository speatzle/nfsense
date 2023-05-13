package firewall

import "encoding/json"

type Rule struct {
	Name    string `json:"name" validate:"required"`
	Match   Match  `json:"match" validate:"required,dive"`
	Comment string `json:"comment,omitempty"`
	Counter bool   `json:"counter,omitempty"`
}

type ForwardRule struct {
	Rule
	Verdict Verdict `json:"verdict" validate:"min=0,max=2"`
}

type Verdict int

const (
	Accept Verdict = iota
	Drop
	Continue
)

func (t Verdict) String() string {
	return [...]string{"accept", "drop", "continue"}[t]
}

func (t *Verdict) FromString(input string) Verdict {
	return map[string]Verdict{
		"accept":   Accept,
		"drop":     Drop,
		"continue": Continue,
	}[input]
}

func (t Verdict) MarshalJSON() ([]byte, error) {
	return json.Marshal(t.String())
}

func (t *Verdict) UnmarshalJSON(b []byte) error {
	var s string
	err := json.Unmarshal(b, &s)
	if err != nil {
		return err
	}
	*t = t.FromString(s)
	return nil
}
