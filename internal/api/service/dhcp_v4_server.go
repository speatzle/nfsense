package service

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/service"
)

type GetDHCPv4ServerParameters struct {
	ID uint
}

type GetDHCPv4ServerResult struct {
	service.DHCPv4Server
}

func (f *Service) GetDHCPv4Server(ctx context.Context, params GetDHCPv4ServerParameters) (GetDHCPv4ServerResult, error) {
	if int(params.ID) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv4Servers) {
		return GetDHCPv4ServerResult{}, fmt.Errorf("DHCPv4Server does not Exist")
	}

	return GetDHCPv4ServerResult{
		DHCPv4Server: f.ConfigManager.GetPendingConfig().Service.DHCPv4Servers[params.ID],
	}, nil
}

type GetDHCPv4ServersResult struct {
	DHCPv4Servers []service.DHCPv4Server `json:"dhcp_v4_servers"`
}

func (f *Service) GetDHCPv4Servers(ctx context.Context, params struct{}) (GetDHCPv4ServersResult, error) {
	return GetDHCPv4ServersResult{
		DHCPv4Servers: f.ConfigManager.GetPendingConfig().Service.DHCPv4Servers,
	}, nil
}

type CreateDHCPv4ServerParameters struct {
	service.DHCPv4Server
}

func (f *Service) CreateDHCPv4Server(ctx context.Context, params CreateDHCPv4ServerParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv4Servers = append(conf.Service.DHCPv4Servers, params.DHCPv4Server)
	return struct{}{}, t.Commit()
}

type UpdateDHCPv4ServerParameters struct {
	Index        uint64               `json:"index"`
	DHCPv4Server service.DHCPv4Server `json:"dhcp_v4_server"`
}

func (f *Service) UpdateDHCPv4Server(ctx context.Context, params UpdateDHCPv4ServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv4Servers) {
		return struct{}{}, fmt.Errorf("DHCPv4Server does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv4Servers[params.Index] = params.DHCPv4Server
	return struct{}{}, t.Commit()
}

type DeleteDHCPv4ServerParameters struct {
	Index uint64 `json:"index"`
}

func (f *Service) DeleteDHCPv4Server(ctx context.Context, params DeleteDHCPv4ServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DHCPv4Servers) {
		return struct{}{}, fmt.Errorf("DHCPv4Server does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DHCPv4Servers = append(conf.Service.DHCPv4Servers[:params.Index], conf.Service.DHCPv4Servers[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
