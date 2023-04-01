package config

import (
	"fmt"

	"golang.org/x/exp/slog"
	"nfsense.net/nfsense/internal/definitions"
)

// ApplyPendingChanges Takes all pending Changes and Tries to Apply them using the Registered Apply Functions.
// In Case of error it Attempts to Revert to the Current Config
func (m *ConfigManager) ApplyPendingChanges() error {
	slog.Info("Applying Pending Changes...")
	for _, fn := range m.applyFunctions {
		err := fn(*m.pendingConfig)
		if err != nil {
			slog.Error("Applying Pending Changes", err)
			err2 := revertToCurrent(m)
			if err2 != nil {
				slog.Error("Reverting Error", err2)
				return fmt.Errorf("Apply Error %w; Reverting Error %w", err, err2)
			}
			return err
		}
	}
	return nil
}

func revertToCurrent(m *ConfigManager) error {
	for _, fn := range m.applyFunctions {
		err := fn(*m.currentConfig)
		if err != nil {
			return err
		}
	}
	return nil
}

func (m *ConfigManager) RegisterApplyFunction(fn func(definitions.Config) error) {
	m.applyFunctions = append(m.applyFunctions, fn)
}
