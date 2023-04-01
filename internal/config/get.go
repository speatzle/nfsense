package config

import "nfsense.net/nfsense/internal/definitions"

func (m *ConfigManager) GetCurrentConfig() definitions.Config {
	return *m.currentConfig.Clone()
}

func (m *ConfigManager) GetPendingConfig() definitions.Config {
	return *m.pendingConfig.Clone()
}
