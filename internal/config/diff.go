package config

import "github.com/r3labs/diff/v3"

func (m *ConfigManager) AreChangesPending() bool {
	return diff.Changed(m.currentConfig, m.pendingConfig)
}

func (m *ConfigManager) GetPendingChangelog() (diff.Changelog, error) {
	return diff.Diff(m.currentConfig, m.pendingConfig, diff.SliceOrdering(true))
}
