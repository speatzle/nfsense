package config

import (
	"encoding/json"
	"fmt"
	"os"

	"nfsense.net/nfsense/internal/definitions/config"
)

func (m *ConfigManager) LoadCurrentConfigFromDisk() error {
	var conf config.Config
	configFile, err := os.Open(m.currentConfigFilePath)
	if err != nil {
		return fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()

	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&conf)
	if err != nil {
		return fmt.Errorf("decoding Config File %w", err)
	}

	err = config.ValidateConfig(&conf)
	if err != nil {
		return fmt.Errorf("validating Config: %w", err)
	}

	m.currentConfig = &conf
	return nil
}

func (m *ConfigManager) LoadPendingConfigFromDisk() error {
	var conf config.Config
	configFile, err := os.Open(m.pendingConfigFilePath)
	if err != nil {
		return fmt.Errorf("opening Config File %w", err)
	}
	defer configFile.Close()

	jsonParser := json.NewDecoder(configFile)
	jsonParser.DisallowUnknownFields()
	err = jsonParser.Decode(&conf)
	if err != nil {
		return fmt.Errorf("decoding Config File %w", err)
	}

	err = config.ValidateConfig(&conf)
	if err != nil {
		return fmt.Errorf("validating Config: %w", err)
	}

	m.pendingConfig = &conf
	return nil
}
