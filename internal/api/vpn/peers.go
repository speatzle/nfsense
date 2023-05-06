package vpn

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/vpn"
)

type GetWireguardPeerParameters struct {
	ID string
}

type GetWireguardPeerResult struct {
	Name string `json:"name"`
	vpn.WireguardPeer
}

func (f *VPN) GetWireguardPeer(ctx context.Context, params GetWireguardPeerParameters) (GetWireguardPeerResult, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers[params.ID]
	if !ok {
		return GetWireguardPeerResult{}, fmt.Errorf("WireguardPeer does not Exist")
	}

	return GetWireguardPeerResult{
		Name:          params.ID,
		WireguardPeer: f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers[params.ID],
	}, nil
}

type GetWireguardPeersResult struct {
	WireguardPeers map[string]vpn.WireguardPeer
}

func (f *VPN) GetWireguardPeers(ctx context.Context, params struct{}) (GetWireguardPeersResult, error) {
	return GetWireguardPeersResult{
		WireguardPeers: f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers,
	}, nil
}

type CreateWireguardPeerParameters struct {
	Name string `json:"name"`
	vpn.WireguardPeer
}

func (f *VPN) CreateWireguardPeer(ctx context.Context, params CreateWireguardPeerParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers[params.Name]
	if ok {
		return struct{}{}, fmt.Errorf("WireguardPeer already Exists")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.VPN.Wireguard.Peers[params.Name] = params.WireguardPeer
	return struct{}{}, t.Commit()
}

type UpdateWireguardPeerParameters struct {
	Name string
	vpn.WireguardPeer
}

func (f *VPN) UpdateWireguardPeer(ctx context.Context, params UpdateWireguardPeerParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("WireguardPeer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.VPN.Wireguard.Peers[params.Name] = params.WireguardPeer
	return struct{}{}, t.Commit()
}

type DeleteWireguardPeerParameters struct {
	Name string
}

func (f *VPN) DeleteWireguardPeer(ctx context.Context, params DeleteWireguardPeerParameters) (struct{}, error) {
	_, ok := f.ConfigManager.GetPendingConfig().VPN.Wireguard.Peers[params.Name]
	if !ok {
		return struct{}{}, fmt.Errorf("WireguardPeer does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	delete(conf.VPN.Wireguard.Peers, params.Name)
	return struct{}{}, t.Commit()
}
