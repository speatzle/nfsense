package network

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetInterfacesParameters struct {
}

type GetInterfacesResult struct {
	Interfaces map[string]definitions.Interface
}

func (f *Network) GetInterfaces(ctx context.Context, params GetInterfacesParameters) (GetInterfacesResult, error) {
	return GetInterfacesResult{
		Interfaces: f.Conf.Network.Interfaces,
	}, nil
}
