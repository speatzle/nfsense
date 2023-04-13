package service

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/service"
)

type GetDHCPv6ServerParameters struct {
	ID uint
}

type GetDHCPv6ServerResult struct {
	service.DHCPv6Server
}

func (f *Service) GetDHCPv6Server(ctx context.Context, params GetDHCPv6ServerParameters) (GetDHCPv6ServerResult, error) {
	if int(params.ID) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv6Servers) {
		return GetDHCPv6ServerResult{}, fmt.Errorf("DHCPv6Server does not Exist")
	}

	return GetDHCPv6ServerResult{
		DHCPv6Server: f.ConfigManager.GetPendingConfig().Service.DHCPv6Servers[params.ID],
	}, nil
}

type GetDHCPv6ServersResult struct {
	DHCPv6Servers []service.DHCPv6Server `json:"dhcp_v6_servers"`
}

func (f *Service) GetDHCPv6Servers(ctx context.Context, params struct{}) (GetDHCPv6ServersResult, error) {
	return GetDHCPv6ServersResult{
		DHCPv6Servers: f.ConfigManager.GetPendingConfig().Service.DHCPv6Servers,
	}, nil
}

type CreateDHCPv6ServerParameters struct {
	service.DHCPv6Server
}

func (f *Service) CreateDHCPv6Server(ctx context.Context, params CreateDHCPv6ServerParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv6Servers = append(conf.Service.DHCPv6Servers, params.DHCPv6Server)
	return struct{}{}, t.Commit()
}

type UpdateDHCPv6ServerParameters struct {
	Index        uint64               `json:"index"`
	DHCPv6Server service.DHCPv6Server `json:"dhcp_v6_server"`
}

func (f *Service) UpdateDHCPv6Server(ctx context.Context, params UpdateDHCPv6ServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv6Servers) {
		return struct{}{}, fmt.Errorf("DHCPv6Server does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv6Servers[params.Index] = params.DHCPv6Server
	return struct{}{}, t.Commit()
}

type DeleteDHCPv6ServerParameters struct {
	Index uint64 `json:"index"`
}

func (f *Service) DeleteDHCPv6Server(ctx context.Context, params DeleteDHCPv6ServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv6Servers) {
		return struct{}{}, fmt.Errorf("DHCPv6Server does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv6Servers = append(conf.Service.DHCPv6Servers[:params.Index], conf.Service.DHCPv6Servers[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
