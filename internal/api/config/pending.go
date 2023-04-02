package config

import (
	"context"
	"fmt"

	"github.com/r3labs/diff/v3"
)

type GetPendingStatusResult struct {
	Changed bool
}

func (c *Config) GetPendingStatus(ctx context.Context, params struct{}) (GetPendingStatusResult, error) {
	return GetPendingStatusResult{
		Changed: c.ConfigManager.AreChangesPending(),
	}, nil
}

type GetPendingChangelogResult struct {
	Changelog diff.Changelog
}

func (c *Config) GetPendingChangelog(ctx context.Context, params struct{}) (GetPendingChangelogResult, error) {
	log, err := c.ConfigManager.GetPendingChangelog()
	if err != nil {
		return GetPendingChangelogResult{}, fmt.Errorf("Get Pending changelog %w", err)
	}
	return GetPendingChangelogResult{
		Changelog: log,
	}, nil
}

func (c *Config) ApplyPendingChanges(ctx context.Context, params struct{}) (struct{}, error) {
	return struct{}{}, c.ConfigManager.ApplyPendingChanges()
}

func (c *Config) DiscardPendingChanges(ctx context.Context, params struct{}) (struct{}, error) {
	return struct{}{}, c.ConfigManager.DiscardPendingConfig()
}
