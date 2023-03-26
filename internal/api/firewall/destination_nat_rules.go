package firewall

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetDestinationNATRulesParameters struct {
}

type GetDestinationNATRulesResult struct {
	DestinationNATRules []definitions.DestinationNATRule
}

func (f *Firewall) GetDestinationNATRules(ctx context.Context, params GetForwardRulesParameters) (GetDestinationNATRulesResult, error) {
	return GetDestinationNATRulesResult{
		DestinationNATRules: f.Conf.Firewall.DestinationNATRules,
	}, nil
}
