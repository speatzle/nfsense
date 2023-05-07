package nftables

import (
	"nfsense.net/nfsense/internal/definitions/config"
	"nfsense.net/nfsense/internal/definitions/firewall"
)

func GenerateDestinationNatAction(conf config.Config, rule firewall.DestinationNATRule) string {
	return ""
}

func GenerateSourceNatAction(conf config.Config, rule firewall.SourceNATRule) string {
	return ""
}
