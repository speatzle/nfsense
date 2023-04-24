package service

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/service"
)

type GetDNSServerParameters struct {
	ID uint
}

type GetDNSServerResult struct {
	service.DNSServer
}

func (f *Service) GetDNSServer(ctx context.Context, params GetDNSServerParameters) (GetDNSServerResult, error) {
	if int(params.ID) >= len(f.ConfigManager.GetPendingConfig().Service.DNSServers) {
		return GetDNSServerResult{}, fmt.Errorf("DNSServer does not Exist")
	}

	return GetDNSServerResult{
		DNSServer: f.ConfigManager.GetPendingConfig().Service.DNSServers[params.ID],
	}, nil
}

type GetDNSServersResult struct {
	DNSServers []service.DNSServer `json:"dns_servers"`
}

func (f *Service) GetDNSServers(ctx context.Context, params struct{}) (GetDNSServersResult, error) {
	return GetDNSServersResult{
		DNSServers: f.ConfigManager.GetPendingConfig().Service.DNSServers,
	}, nil
}

type CreateDNSServerParameters struct {
	service.DNSServer
}

func (f *Service) CreateDNSServer(ctx context.Context, params CreateDNSServerParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DNSServers = append(conf.Service.DNSServers, params.DNSServer)
	return struct{}{}, t.Commit()
}

type UpdateDNSServerParameters struct {
	Index     uint64            `json:"index"`
	DNSServer service.DNSServer `json:"dns_server"`
}

func (f *Service) UpdateDNSServer(ctx context.Context, params UpdateDNSServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DNSServers) {
		return struct{}{}, fmt.Errorf("DNSServer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DNSServers[params.Index] = params.DNSServer
	return struct{}{}, t.Commit()
}

type DeleteDNSServerParameters struct {
	Index uint64 `json:"index"`
}

func (f *Service) DeleteDNSServer(ctx context.Context, params DeleteDNSServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.DNSServers) {
		return struct{}{}, fmt.Errorf("DNSServer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.DNSServers = append(conf.Service.DNSServers[:params.Index], conf.Service.DNSServers[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
