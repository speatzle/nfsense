package config

import (
	"sync"

	"nfsense.net/nfsense/internal/definitions/config"
)

type ConfigManager struct {
	currentConfigFilePath string
	pendingConfigFilePath string

	currentConfig *config.Config
	pendingConfig *config.Config

	transactionMutex sync.Mutex

	applyFunctions []func(currentConfig config.Config, pendingConfig config.Config) error
}

func CreateConfigManager() *ConfigManager {
	manager := ConfigManager{
		currentConfigFilePath: "config.json",
		pendingConfigFilePath: "pending.json",
		currentConfig:         &config.Config{},
		pendingConfig:         &config.Config{},
	}
	return &manager
}
