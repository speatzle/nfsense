package config

import (
	"sync"

	"nfsense.net/nfsense/internal/definitions"
)

type ConfigManager struct {
	currentConfigFilePath string
	pendingConfigFilePath string

	currentConfig *definitions.Config
	pendingConfig *definitions.Config

	transactionMutex sync.Mutex

	applyFunctions []func(definitions.Config) error
}

func CreateConfigManager() *ConfigManager {
	manager := ConfigManager{
		currentConfigFilePath: "config.json",
		pendingConfigFilePath: "pending.json",
		currentConfig:         &definitions.Config{},
		pendingConfig:         &definitions.Config{},
	}
	return &manager
}
