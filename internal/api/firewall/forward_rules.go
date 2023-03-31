package firewall

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetForwardRulesResult struct {
	ForwardRules []definitions.ForwardRule
}

func (f *Firewall) GetForwardRules(ctx context.Context, params struct{}) (GetForwardRulesResult, error) {
	return GetForwardRulesResult{
		ForwardRules: f.Conf.Firewall.ForwardRules,
	}, nil
}
