package common

import (
	"encoding/json"
	"errors"
	"time"
)

type Duration struct {
	time.Duration
}

// MarshalJSON for IPNet
func (i Duration) MarshalJSON() ([]byte, error) {
	return json.Marshal(int(i.Seconds()))
}

// UnmarshalJSON for IPNet
func (i *Duration) UnmarshalJSON(b []byte) error {
	var v interface{}
	if err := json.Unmarshal(b, &v); err != nil {
		return err
	}
	switch value := v.(type) {
	case float64:
		i.Duration = time.Second * time.Duration(value)
		return nil
	case int:
		i.Duration = time.Second * time.Duration(value)
		return nil
	default:
		return errors.New("invalid duration")
	}
}
