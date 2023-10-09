package validation

import (
	"encoding/json"
	"fmt"
)

func ValidateConfig(conf any) error {

	// TODO find a better way validate config since jsonschema only takes a map[string]interface{}
	data, err := json.Marshal(conf)
	if err != nil {
		panic(fmt.Errorf("Marshal Error: %w", err))
	}
	var clone any
	err = json.Unmarshal(data, &clone)
	if err != nil {
		panic(fmt.Errorf("Unmarshal Error: %w", err))
	}

	return schema.Validate(clone)
}
