package config

import (
	"encoding/json"
	"fmt"
	"os"

	"nfsense.net/nfsense/internal/definitions"
)

func (m *ConfigManager) LoadCurrentConfigFromDisk() error {
	var config definitions.Config
	configFile, err := os.Open(m.currentConfigFilePath)
	if err != nil {
		return fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()

	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&config)
	if err != nil {
		return fmt.Errorf("decoding Config File %w", err)
	}

	err = definitions.ValidateConfig(&config)
	if err != nil {
		return fmt.Errorf("validating Config: %w", err)
	}

	m.currentConfig = &config
	return nil
}

func (m *ConfigManager) LoadPendingConfigFromDisk() error {
	var config definitions.Config
	configFile, err := os.Open(m.pendingConfigFilePath)
	if err != nil {
		return fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()

	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&config)
	if err != nil {
		return fmt.Errorf("decoding Config File %w", err)
	}

	err = definitions.ValidateConfig(&config)
	if err != nil {
		return fmt.Errorf("validating Config: %w", err)
	}

	m.pendingConfig = &config
	return nil
}
