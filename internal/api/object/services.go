package object

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetServicesResult struct {
	Services map[string]definitions.Service
}

func (f *Object) GetServices(ctx context.Context, params struct{}) (GetServicesResult, error) {
	return GetServicesResult{
		Services: f.ConfigManager.GetPendingConfig().Object.Services,
	}, nil
}

type CreateServiceParameters struct {
	Name    string
	Service definitions.Service
}

func (f *Object) CreateService(ctx context.Context, params CreateServiceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Services[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("Service already Exists")
	}

	f.ConfigManager.GetPendingConfig().Object.Services[params.Name] = params.Service
	return struct{}{}, nil
}

type UpdateServiceParameters struct {
	Name    string
	Service definitions.Service
}

func (f *Object) UpdateService(ctx context.Context, params CreateServiceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Services[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Service does not Exist")
	}

	f.ConfigManager.GetPendingConfig().Object.Services[params.Name] = params.Service
	return struct{}{}, nil
}

type DeleteServiceParameters struct {
	Name string
}

func (f *Object) DeleteService(ctx context.Context, params DeleteServiceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().Object.Services[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("Interface does not Exist")
	}

	delete(f.ConfigManager.GetPendingConfig().Object.Services, params.Name)
	return struct{}{}, nil
}
