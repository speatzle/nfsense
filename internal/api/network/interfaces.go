package network

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
	"nfsense.net/nfsense/internal/interfaces"
)

type GetInterfacesResult struct {
	Interfaces map[string]definitions.Interface
}

func (f *Network) GetInterfaces(ctx context.Context, params struct{}) (GetInterfacesResult, error) {
	return GetInterfacesResult{
		Interfaces: f.Conf.Network.Interfaces,
	}, nil
}

type DeleteInterfaceParameters struct {
	Interface string
}

func (f *Network) DeleteInterface(ctx context.Context, params DeleteInterfaceParameters) (struct{}, error) {
	_, ok := f.Conf.Network.Interfaces[params.Interface]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	delete(f.Conf.Network.Interfaces, params.Interface)
	return struct{}{}, nil
}

type ApplyInterfacesResult struct {
	Log string
}

func (f *Network) ApplyInterfaces(ctx context.Context, params struct{}) (ApplyInterfacesResult, error) {
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
