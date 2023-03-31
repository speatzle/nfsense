package object

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetAddressesResult struct {
	Addresses map[string]definitions.Address
}

func (f *Object) GetAddresses(ctx context.Context, params struct{}) (GetAddressesResult, error) {
	return GetAddressesResult{
		Addresses: f.Conf.Object.Addresses,
	}, nil
}
