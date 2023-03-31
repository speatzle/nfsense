package network

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
	"nfsense.net/nfsense/internal/interfaces"
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

type ApplyInterfacesParameters struct {
}

type ApplyInterfacesResult struct {
	Log string
}

func (f *Network) ApplyInterfaces(ctx context.Context, params ApplyInterfacesParameters) (ApplyInterfacesResult, error) {
	data, err := interfaces.GenerateInterfacesFile(*f.Conf)
	if err != nil {
		return ApplyInterfacesResult{}, fmt.Errorf("Generating Interfaces File: %w", err)
	}
	log, err := interfaces.ApplyInterfacesFile(data)
	if err != nil {
		return ApplyInterfacesResult{}, fmt.Errorf("Applying Interfaces File: %w", err)
	}
	return ApplyInterfacesResult{
		Log: log,
	}, nil
}
