package service

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/service"
)

type GetNTPServerParameters struct {
	ID uint
}

type GetNTPServerResult struct {
	service.NTPServer
}

func (f *Service) GetNTPServer(ctx context.Context, params GetNTPServerParameters) (GetNTPServerResult, error) {
	if int(params.ID) >= len(f.ConfigManager.GetPendingConfig().Service.NTPServers) {
		return GetNTPServerResult{}, fmt.Errorf("NTPServer does not Exist")
	}

	return GetNTPServerResult{
		NTPServer: f.ConfigManager.GetPendingConfig().Service.NTPServers[params.ID],
	}, nil
}

type GetNTPServersResult struct {
	NTPServers []service.NTPServer `json:"ntp_servers"`
}

func (f *Service) GetNTPServers(ctx context.Context, params struct{}) (GetNTPServersResult, error) {
	return GetNTPServersResult{
		NTPServers: f.ConfigManager.GetPendingConfig().Service.NTPServers,
	}, nil
}

type CreateNTPServerParameters struct {
	service.NTPServer
}

func (f *Service) CreateNTPServer(ctx context.Context, params CreateNTPServerParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.NTPServers = append(conf.Service.NTPServers, params.NTPServer)
	return struct{}{}, t.Commit()
}

type UpdateNTPServerParameters struct {
	Index     uint64            `json:"index"`
	NTPServer service.NTPServer `json:"ntp_server"`
}

func (f *Service) UpdateNTPServer(ctx context.Context, params UpdateNTPServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.NTPServers) {
		return struct{}{}, fmt.Errorf("NTPServer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.NTPServers[params.Index] = params.NTPServer
	return struct{}{}, t.Commit()
}

type DeleteNTPServerParameters struct {
	Index uint64 `json:"index"`
}

func (f *Service) DeleteNTPServer(ctx context.Context, params DeleteNTPServerParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Service.NTPServers) {
		return struct{}{}, fmt.Errorf("NTPServer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Service.NTPServers = append(conf.Service.NTPServers[:params.Index], conf.Service.NTPServers[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
