package dhcp

import (
	"bytes"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/network"
)

type DHCPServerInterfaces struct {
	V4 []string
	V6 []string
}

func GenerateDHCPServerDefaultsConfiguration(conf config.Config) (string, error) {
	v4 := []string{}
	for _, s := range conf.Service.DHCPv4Servers {
		if conf.Network.Interfaces[s.Interface].Type == network.Hardware {
			v4 = append(v4, *conf.Network.Interfaces[s.Interface].HardwareDevice)
		} else {
			v4 = append(v4, s.Interface)
		}
	}
	v6 := []string{}
	for _, s := range conf.Service.DHCPv6Servers {
		if conf.Network.Interfaces[s.Interface].Type == network.Hardware {
			v6 = append(v6, *conf.Network.Interfaces[s.Interface].HardwareDevice)
		} else {
			v6 = append(v6, s.Interface)
		}
	}
	interfaces := DHCPServerInterfaces{
		V4: v4,
		V6: v6,
	}
	buf := new(bytes.Buffer)
	err := templates.ExecuteTemplate(buf, "default.tmpl", interfaces)
	if err != nil {
		return "", fmt.Errorf("executing default.tmpl template: %w", err)
	}
	return buf.String(), nil
}
