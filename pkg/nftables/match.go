package nftables

import (
	"fmt"

	"github.con/speatzle/nfsense/pkg/definitions"
	"github.con/speatzle/nfsense/pkg/util"
)

func GenerateMatcher(services *map[string]definitions.Service, addresses *map[string]definitions.Address, match definitions.Match) (string, error) {
	return GenerateServiceMatcher(services, match), nil
}

func GenerateServiceMatcher(allServices *map[string]definitions.Service, match definitions.Match) string {
	serviceList := util.ResolveBaseServices(*allServices, match.Services)

	tcpSPorts := []string{}
	tcpDPorts := []string{}
	udpSPorts := []string{}
	udpDPorts := []string{}
	icmpCodes := []string{}

	for _, service := range serviceList {
		switch service.Type {
		case definitions.TCP:
			if service.GetSPort() != "0" {
				tcpSPorts = append(tcpSPorts, service.GetSPort())
			}
			if service.GetDPort() != "0" {
				tcpDPorts = append(tcpDPorts, service.GetDPort())
			}
		case definitions.UDP:
			if service.GetSPort() != "0" {
				udpSPorts = append(udpSPorts, service.GetSPort())
			}
			if service.GetDPort() != "0" {
				udpDPorts = append(udpDPorts, service.GetDPort())
			}
		case definitions.ICMP:
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
