package config

func (m *ConfigManager) DiscardPendingConfig() error {
	m.pendingConfig = m.currentConfig.Clone()
	return nil
}
