package network

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetStaticRoutesResult struct {
	StaticRoutes []definitions.StaticRoute
}

func (f *Network) GetStaticRoutes(ctx context.Context, params struct{}) (GetStaticRoutesResult, error) {
	return GetStaticRoutesResult{
		StaticRoutes: f.ConfigManager.GetPendingConfig().Network.StaticRoutes,
	}, nil
}

func (f *Network) CreateStaticRoute(ctx context.Context, params definitions.StaticRoute) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.StaticRoutes = append(conf.Network.StaticRoutes, params)
	return struct{}{}, t.Commit()
}

type UpdateStaticRouteParameters struct {
	Index uint
	definitions.StaticRoute
}

func (f *Network) UpdateStaticRoute(ctx context.Context, params UpdateStaticRouteParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules) {
		return struct{}{}, fmt.Errorf("StaticRoute does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.StaticRoutes = append(conf.Network.StaticRoutes, params.StaticRoute)
	return struct{}{}, t.Commit()
}

type DeleteStaticRouteParameters struct {
	Index uint
}

func (f *Network) DeleteStaticRoute(ctx context.Context, params DeleteStaticRouteParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules) {
		return struct{}{}, fmt.Errorf("StaticRoute does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.StaticRoutes = append(conf.Network.StaticRoutes[:params.Index], conf.Network.StaticRoutes[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
