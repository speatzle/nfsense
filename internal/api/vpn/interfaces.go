package vpn

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/vpn"
)

type GetWireguardInterfaceParameters struct {
	ID string
}

type GetWireguardInterfaceResult struct {
	Name string `json:"name"`
	vpn.WireguardInterface
}

func (f *VPN) GetWireguardInterface(ctx context.Context, params GetWireguardInterfaceParameters) (GetWireguardInterfaceResult, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces[params.ID]
	if !ok {
		return GetWireguardInterfaceResult{}, fmt.Errorf("WireguardInterface does not Exist")
	}

	return GetWireguardInterfaceResult{
		Name:               params.ID,
		WireguardInterface: f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces[params.ID],
	}, nil
}

type GetWireguardInterfacesResult struct {
	Interfaces map[string]vpn.WireguardInterface
}

func (f *VPN) GetWireguardInterfaces(ctx context.Context, params struct{}) (GetWireguardInterfacesResult, error) {
	return GetWireguardInterfacesResult{
		Interfaces: f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces,
	}, nil
}

type CreateWireguardInterfaceParameters struct {
	Name string `json:"name"`
	vpn.WireguardInterface
}

func (f *VPN) CreateWireguardInterface(ctx context.Context, params CreateWireguardInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("WireguardInterface already Exists")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.VPN.Wireguard.Interfaces[params.Name] = params.WireguardInterface
	return struct{}{}, t.Commit()
}

type UpdateWireguardInterfaceParameters struct {
	Name string
	vpn.WireguardInterface
}

func (f *VPN) UpdateWireguardInterface(ctx context.Context, params UpdateWireguardInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("WireguardInterface does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.VPN.Wireguard.Interfaces[params.Name] = params.WireguardInterface
	return struct{}{}, t.Commit()
}

type DeleteWireguardInterfaceParameters struct {
	Name string
}

func (f *VPN) DeleteWireguardInterface(ctx context.Context, params DeleteWireguardInterfaceParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Interfaces[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("WireguardInterface does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	delete(conf.VPN.Wireguard.Interfaces, params.Name)
	return struct{}{}, t.Commit()
}
