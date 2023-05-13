package nftables

import (
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/object"
)

func GenerateDestinationNatAction(conf config.Config, rule firewall.DestinationNATRule) string {
	destination := ""

	if rule.Address != nil {
		addr := conf.Object.Addresses[*rule.Address]

		if addr.Type == object.Host {
			destination = addr.Host.String()
		} else {
			panic("invalid address type")
		}
	}

	if rule.Service != nil {
		serv := conf.Object.Services[*rule.Service]

		if serv.Type == object.TCP || serv.Type == object.UDP {
			destination += ":" + serv.GetDPort()
		} else {
			panic("invalid service type")
		}
	}
	return "dnat to " + destination
}

func GenerateSourceNatAction(conf config.Config, rule firewall.SourceNATRule) string {
	if rule.Type == firewall.Masquerade {
		return "masqerade"
	}

	source := ""

	if rule.Address != nil {
		addr := conf.Object.Addresses[*rule.Address]

		if addr.Type == object.Host {
			source = addr.Host.String()
		} else if addr.Type == object.Range {
			source = addr.Range.String()
		} else {
			panic("invalid address type")
		}
	}

	if rule.Service != nil {
		serv := conf.Object.Services[*rule.Service]

		if serv.Type == object.TCP || serv.Type == object.UDP {
			source += ":" + serv.GetSPort()
		} else {
			panic("invalid service type")
		}
	}

	return "snat to " + source
}
