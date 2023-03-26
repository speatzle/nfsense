package firewall

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetServicesParameters struct {
}

type GetServicesResult struct {
	Services map[string]definitions.Service
}

func (f *Firewall) GetServices(ctx context.Context, params GetForwardRulesParameters) (GetServicesResult, error) {
	return GetServicesResult{
		Services: f.Conf.Firewall.Services,
	}, nil
}
