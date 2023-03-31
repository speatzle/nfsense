package object

import (
	"context"

	"nfsense.net/nfsense/internal/definitions"
)

type GetServicesResult struct {
	Services map[string]definitions.Service
}

func (f *Object) GetServices(ctx context.Context, params struct{}) (GetServicesResult, error) {
	return GetServicesResult{
		Services: f.Conf.Object.Services,
	}, nil
}
