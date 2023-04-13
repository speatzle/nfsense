package config

import (
	"encoding/json"
	"fmt"
	"os"

	"nfsense.net/nfsense/internal/definitions/config"
)

func (m *ConfigManager) saveConfig(path string, conf *config.Config) error {
	data, err := json.MarshalIndent(conf, "", "	")
	if err != nil {
		return fmt.Errorf("Marshal Config: %w", err)
	}

	err = os.WriteFile(path, data, 0644)
	if err != nil {
		return fmt.Errorf("Write Config: %w", err)
	}

	return nil
}
