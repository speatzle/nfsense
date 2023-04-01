package firewall

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetSourceNATRulesResult struct {
	SourceNATRules []definitions.SourceNATRule
}

func (f *Firewall) GetSourceNATRules(ctx context.Context, params struct{}) (GetSourceNATRulesResult, error) {
	return GetSourceNATRulesResult{
		SourceNATRules: f.ConfigManager.GetPendingConfig().Firewall.SourceNATRules,
	}, nil
}
