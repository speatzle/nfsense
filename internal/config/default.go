package config

import (
	"fmt"

	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/network"
	"nfsense.net/nfsense/internal/definitions/object"
	"nfsense.net/nfsense/internal/definitions/service"
	"nfsense.net/nfsense/internal/definitions/system"
	"nfsense.net/nfsense/internal/definitions/vpn"
)

func (m *ConfigManager) LoadDefaultConfig() error {
	conf := config.Config{
		ConfigVersion: 1,
		Firewall: firewall.Firewall{
			ForwardRules:        []firewall.ForwardRule{},
			DestinationNATRules: []firewall.DestinationNATRule{},
			SourceNATRules:      []firewall.SourceNATRule{},
		},
		Object: object.Object{
			Addresses: map[string]object.Address{},
			Services:  map[string]object.Service{},
		},
		Network: network.Network{
			Interfaces:   map[string]network.Interface{},
			StaticRoutes: []network.StaticRoute{},
		},
		Service: service.Service{
			DHCPv4Servers: []service.DHCPv4Server{},
			DHCPv6Servers: []service.DHCPv6Server{},
			DNSServers:    []service.DNSServer{},
			NTPServers:    []service.NTPServer{},
		},
		VPN: vpn.VPN{
			Wireguard: vpn.Wireguard{
				Interfaces: map[string]vpn.WireguardInterface{},
				Peers:      map[string]vpn.WireguardPeer{},
			},
		},
		System: system.System{
			Users: map[string]system.User{},
		},
	}

	err := config.ValidateConfig(&conf)
	if err != nil {
		return fmt.Errorf("validating Config: %w", err)
	}

	m.currentConfig = &conf
	m.pendingConfig = &conf
	return nil
}
