package nftables

import (
	"fmt"

	"nfsense.net/nfsense/internal/definitions/firewall"
	"nfsense.net/nfsense/internal/definitions/object"
	"nfsense.net/nfsense/internal/util"
)

func GenerateMatcher(services map[string]object.Service, addresses map[string]object.Address, match firewall.Match) (string, error) {
	return GenerateAddressMatcher(addresses, match) + " " + GenerateServiceMatcher(services, match), nil
}

func GenerateServiceMatcher(allServices map[string]object.Service, match firewall.Match) string {
	serviceList := util.ResolveBaseServices(allServices, match.Services)

	tcpSPorts := []string{}
	tcpDPorts := []string{}
	udpSPorts := []string{}
	udpDPorts := []string{}
	icmpCodes := []string{}

	for _, service := range serviceList {
		switch service.Type {
		case object.TCP:
			if service.GetSPort() != "" {
				tcpSPorts = append(tcpSPorts, service.GetSPort())
			}
			if service.GetDPort() != "" {
				tcpDPorts = append(tcpDPorts, service.GetDPort())
			}
		case object.UDP:
			if service.GetSPort() != "" {
				udpSPorts = append(udpSPorts, service.GetSPort())
			}
			if service.GetDPort() != "" {
				udpDPorts = append(udpDPorts, service.GetDPort())
			}
		case object.ICMP:
			icmpCodes = append(icmpCodes, fmt.Sprint(service.ICMPCode))
		default:
			panic("invalid service type")
		}
	}

	res := ""

	if len(tcpSPorts) != 0 {
		res += "tcp sport " + util.ConvertSliceToSetString(tcpSPorts) + " "
	}
	if len(tcpDPorts) != 0 {
		res += "tcp dport " + util.ConvertSliceToSetString(tcpDPorts) + " "
	}
	if len(udpSPorts) != 0 {
		res += "udp sport " + util.ConvertSliceToSetString(udpSPorts) + " "
	}
	if len(udpDPorts) != 0 {
		res += "udp dport " + util.ConvertSliceToSetString(udpDPorts) + " "
	}
	if len(icmpCodes) != 0 {
		res += "icmp codes " + util.ConvertSliceToSetString(icmpCodes) + " "
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
