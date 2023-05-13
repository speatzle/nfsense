package network

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/network"
)

type GetStaticRouteParameters struct {
	ID uint
}

type GetStaticRouteResult struct {
	network.StaticRoute
}

func (f *Network) GetStaticRoute(ctx context.Context, params GetStaticRouteParameters) (GetStaticRouteResult, error) {
	if int(params.ID) >= len(f.ConfigManager.GetPendingConfig().Network.StaticRoutes) {
		return GetStaticRouteResult{}, fmt.Errorf("StaticRoute does not Exist")
	}

	return GetStaticRouteResult{
		StaticRoute: f.ConfigManager.GetPendingConfig().Network.StaticRoutes[params.ID],
	}, nil
}

type GetStaticRoutesResult struct {
	StaticRoutes []network.StaticRoute
}

func (f *Network) GetStaticRoutes(ctx context.Context, params struct{}) (GetStaticRoutesResult, error) {
	return GetStaticRoutesResult{
		StaticRoutes: f.ConfigManager.GetPendingConfig().Network.StaticRoutes,
	}, nil
}

func (f *Network) CreateStaticRoute(ctx context.Context, params network.StaticRoute) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Network.StaticRoutes = append(conf.Network.StaticRoutes, params)
	return struct{}{}, t.Commit()
}

type UpdateStaticRouteParameters struct {
	Index uint
	network.StaticRoute
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
