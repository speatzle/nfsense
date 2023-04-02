package config

import (
	"encoding/json"
	"fmt"
	"io/ioutil"

	"nfsense.net/nfsense/internal/definitions"
)

func (m *ConfigManager) saveConfig(path string, conf *definitions.Config) error {
	data, err := json.MarshalIndent(conf, "", "	")
	if err != nil {
		return fmt.Errorf("Marshal Config: %w", err)
	}

	err = ioutil.WriteFile(path, data, 0644)
	if err != nil {
		return fmt.Errorf("Write Config: %w", err)
	}

	return nil
}
