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

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Object.Addresses[params.Name] = params.Address
	return struct{}{}, t.Commit()
}

type UpdateAddressParameters struct {
	Name    string
	Address definitions.Address
}

func (f *Object) UpdateAddress(ctx context.Context, params UpdateAddressParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Address does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Object.Addresses[params.Name] = params.Address
	return struct{}{}, t.Commit()
}

type DeleteAddressParameters struct {
	Name string
}

func (f *Object) DeleteAddress(ctx context.Context, params DeleteAddressParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Addresses[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	delete(conf.Object.Addresses, params.Name)
	return struct{}{}, t.Commit()
}
