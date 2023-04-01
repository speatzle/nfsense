package object

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetAddressesResult struct {
	Addresses map[string]definitions.Address
}

func (f *Object) GetAddresses(ctx context.Context, params struct{}) (GetAddressesResult, error) {
	return GetAddressesResult{
		Addresses: f.ConfigManager.GetPendingConfig().Object.Addresses,
	}, nil
}

type CreateAddressParameters struct {
	Name    string
	Address definitions.Address
}

func (f *Object) CreateAddress(ctx context.Context, params CreateAddressParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("Address already Exists")
	}

	f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name] = params.Address
	return struct{}{}, nil
}

type UpdateAddressParameters struct {
	Name    string
	Address definitions.Address
}

func (f *Object) UpdateAddress(ctx context.Context, params CreateAddressParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Address does not Exist")
	}

	f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name] = params.Address
	return struct{}{}, nil
}

type DeleteAddressParameters struct {
	Name string
}

func (f *Object) DeleteAddress(ctx context.Context, params DeleteAddressParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	delete(f.ConfigManager.GetPendingConfig().Object.Addresses, params.Name)
	return struct{}{}, nil
}
