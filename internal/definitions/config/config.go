package config

import (
	"encoding/json"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/network"
	"nfsense.net/nfsense/internal/definitions/object"
	"nfsense.net/nfsense/internal/definitions/service"
	"nfsense.net/nfsense/internal/definitions/system"
	"nfsense.net/nfsense/internal/definitions/vpn"
	"nfsense.net/nfsense/internal/validation"
)

type Config struct {
	ConfigVersion uint64            `json:"config_version"`
	Firewall      firewall.Firewall `json:"firewall"`
	Object        object.Object     `json:"object"`
	Network       network.Network   `json:"network"`
	Service       service.Service   `json:"service"`
	VPN           vpn.VPN           `json:"vpn"`
	System        system.System     `json:"system"`
}

// Clone TODO find a better way to deep copy
func (c *Config) Clone() *Config {
	data, err := json.Marshal(c)
	if err != nil {
		panic(fmt.Errorf("Marshal Error: %w", err))
	}
	var clone Config
	err = json.Unmarshal(data, &clone)
	if err != nil {
		panic(fmt.Errorf("Unmarshal Error: %w", err))
	}
	return &clone
}

func ValidateConfig(conf *Config) error {
	return validation.ValidateConfig(*conf)
}
