package config

import (
	"nfsense.net/nfsense/internal/definitions/config"
)

func (m *ConfigManager) GetCurrentConfig() config.Config {
	return *m.currentConfig.Clone()
}

func (m *ConfigManager) GetPendingConfig() config.Config {
	return *m.pendingConfig.Clone()
}
