package firewall

import (
	"context"

	"nfsense.net/nfsense/pkg/definitions"
)

type GetAddressesParameters struct {
}

type GetAddressesResult struct {
	Addresses map[string]definitions.Address
}

func (f *Firewall) GetAddresses(ctx context.Context, params GetForwardRulesParameters) (GetAddressesResult, error) {
	return GetAddressesResult{
		Addresses: f.Conf.Firewall.Addresses,
	}, nil
}
