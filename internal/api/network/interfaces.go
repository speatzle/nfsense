package network

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetInterfacesResult struct {
	Interfaces map[string]definitions.Interface
}

func (f *Network) GetInterfaces(ctx context.Context, params struct{}) (GetInterfacesResult, error) {
	return GetInterfacesResult{
		Interfaces: f.ConfigManager.GetPendingConfig().Network.Interfaces,
	}, nil
}

type CreateInterfaceParameters struct {
	Name      string
	Interface definitions.Interface
}

func (f *Network) CreateInterface(ctx context.Context, params CreateInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Network.Interfaces[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("Interface already Exists")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.Interfaces[params.Name] = params.Interface
	return struct{}{}, t.Commit()
}

type UpdateInterfaceParameters struct {
	Name      string
	Interface definitions.Interface
}

func (f *Network) UpdateInterface(ctx context.Context, params UpdateInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Network.Interfaces[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.Interfaces[params.Name] = params.Interface
	return struct{}{}, t.Commit()
}

type DeleteInterfaceParameters struct {
	Name string
}

func (f *Network) DeleteInterface(ctx context.Context, params DeleteInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Network.Interfaces[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	delete(conf.Network.Interfaces, params.Name)
	return struct{}{}, t.Commit()
}
