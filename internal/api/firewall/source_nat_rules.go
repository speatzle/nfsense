package firewall

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetSourceNATRulesParameters struct {
}

type GetSourceNATRulesResult struct {
	SourceNATRules []definitions.SourceNATRule
}

func (f *Firewall) GetSourceNATRules(ctx context.Context, params GetForwardRulesParameters) (GetSourceNATRulesResult, error) {
	return GetSourceNATRulesResult{
		SourceNATRules: f.Conf.Firewall.SourceNATRules,
	}, nil
}
