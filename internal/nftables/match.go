package nftables

import (
	"fmt"

	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/object"
	"nfsense.net/nfsense/internal/util"
)

func GenerateServiceMatcher(service object.Service) string {
	res := ""

	switch service.Type {
	case object.TCP:
		if service.GetSPort() != "" {
			res = "tcp sport " + service.GetSPort()
		}
		if service.GetDPort() != "" {
			if len(res) != 0 {
				res += " "
			}
			res += "tcp dport " + service.GetDPort()
		}
	case object.UDP:
		if service.GetSPort() != "" {
			res = "udp sport " + service.GetSPort()
		}
		if service.GetDPort() != "" {
			if len(res) != 0 {
				res += " "
			}
			res += "udp dport " + service.GetDPort()
		}
	case object.ICMP:
		res = "icmp codes " + fmt.Sprint(service.ICMPCode)
	default:
		panic("invalid service type")
	}

	return res
}

func GenerateAddressMatcher(allAddresses map[string]object.Address, match firewall.Match) string {
	sourceAddressList := util.ResolveBaseAddresses(allAddresses, match.SourceAddresses)
	destinationAddressList := util.ResolveBaseAddresses(allAddresses, match.DestinationAddresses)

	sourceAddresses := []string{}
	destinationAddresses := []string{}

	for _, address := range sourceAddressList {
		switch address.Type {
		case object.Host:
			sourceAddresses = append(sourceAddresses, address.Host.String())
		case object.Range:
			sourceAddresses = append(sourceAddresses, address.Range.String())
		case object.NetworkAddress:
			sourceAddresses = append(sourceAddresses, address.NetworkAddress.String())
		default:
			panic("invalid address type")
		}
	}

	for _, address := range destinationAddressList {
		switch address.Type {
		case object.Host:
			destinationAddresses = append(destinationAddresses, address.Host.String())
		case object.Range:
			destinationAddresses = append(destinationAddresses, address.Range.String())
		case object.NetworkAddress:
			destinationAddresses = append(destinationAddresses, address.NetworkAddress.String())
		default:
			panic("invalid address type")
		}
	}

	res := ""

	if len(sourceAddresses) != 0 {
		res += "ip saddr " + util.ConvertSliceToSetString(sourceAddresses) + " "
	}
	if len(destinationAddresses) != 0 {
		res += "ip daddr " + util.ConvertSliceToSetString(destinationAddresses) + " "
	}

	return res
}
