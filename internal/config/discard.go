package config

import (
	"errors"
	"os"
)

func (m *ConfigManager) DiscardPendingConfig() error {
	m.pendingConfig = m.currentConfig.Clone()

	err := os.Remove(m.pendingConfigFilePath)
	if !errors.Is(err, os.ErrNotExist) {
		return err
	}
	return nil
}
