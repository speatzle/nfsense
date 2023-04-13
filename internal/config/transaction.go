package config

import (
	"fmt"
	"sync"

	"nfsense.net/nfsense/internal/definitions/config"
)

type ConfigTransaction struct {
	finished      bool
	mutex         sync.Mutex
	configManager *ConfigManager
	changes       *config.Config
}

func (m *ConfigManager) StartTransaction() (*ConfigTransaction, *config.Config) {
	m.transactionMutex.Lock()
	confCopy := m.pendingConfig.Clone()
	return &ConfigTransaction{
		configManager: m,
		changes:       confCopy,
	}, confCopy
}

func (t *ConfigTransaction) Commit() error {
	t.mutex.Lock()
	defer t.mutex.Unlock()

	if t.finished {
		return fmt.Errorf("transaction already finished")
	}

	t.finished = true
	defer t.configManager.transactionMutex.Unlock()

	err := config.ValidateConfig(t.changes)
	if err != nil {
		return fmt.Errorf("validating Config before Apply: %w", err)
	}

	err = t.configManager.saveConfig(t.configManager.pendingConfigFilePath, t.changes)
	if err != nil {
		return fmt.Errorf("Save Current Config: %w", err)
	}

	t.configManager.pendingConfig = t.changes.Clone()

	return nil
}

// Discard Discards the Transaction.
// Is a noop if The Transaction Already Finished due to a Commit() or another Discard()
func (t *ConfigTransaction) Discard() {
	t.mutex.Lock()
	defer t.mutex.Unlock()

	if !t.finished {
		t.finished = true
		t.configManager.transactionMutex.Unlock()
	}
}
