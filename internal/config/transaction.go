package config

import (
	"fmt"
	"sync"

	"nfsense.net/nfsense/internal/definitions"
)

type ConfigTransaction struct {
	finished      bool
	mutex         sync.Mutex
	configManager *ConfigManager
	changes       *definitions.Config
}

func (m *ConfigManager) StartTransaction() (*ConfigTransaction, *definitions.Config) {
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

	err := definitions.ValidateConfig(t.changes)
	if err != nil {
		return fmt.Errorf("validating Config before Apply: %w", err)
	}

	t.configManager.pendingConfig = t.changes.Clone()

	return nil
}

func (t *ConfigTransaction) Discard() {
	t.mutex.Lock()
	defer t.mutex.Unlock()

	if !t.finished {
		t.finished = true
		t.configManager.transactionMutex.Unlock()
	}
}
