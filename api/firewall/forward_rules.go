package firewall

import (
	"context"

	"nfsense.net/nfsense/pkg/definitions"
)

type GetForwardRulesParameters struct {
}

type GetForwardRulesResult struct {
	ForwardRules []definitions.ForwardRule
}

func (f *Firewall) GetForwardRules(ctx context.Context, params GetForwardRulesParameters) (GetForwardRulesResult, error) {
	return GetForwardRulesResult{
		ForwardRules: f.Conf.Firewall.ForwardRules,
	}, nil
}
