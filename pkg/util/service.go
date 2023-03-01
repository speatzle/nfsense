package util

import "github.con/speatzle/nfsense/pkg/definitions"

// ResolveBaseServices Resolves all groups to their base Services
func ResolveBaseServices(allServices map[string]definitions.Service, serviceNames []string) []definitions.Service {
	baseServices := []definitions.Service{}

	for _, serviceName := range serviceNames {
		service := allServices[serviceName]

		if service.Type == definitions.ServiceGroup {
			baseServices = append(baseServices, resolveChildren(allServices, service)...)
		} else {
			baseServices = append(baseServices, service)
		}

	}

	return baseServices
}

func resolveChildren(allServices map[string]definitions.Service, s definitions.Service) []definitions.Service {
	serviceList := []definitions.Service{}
	for _, serviceName := range *s.Children {
		service := allServices[serviceName]

		if service.Type == definitions.ServiceGroup {
			serviceList = append(serviceList, resolveChildren(allServices, service)...)
		} else {
			serviceList = append(serviceList, service)
		}
	}
	return serviceList
}

func ConvertSliceToSetString(slice []string) string {
	if len(slice) == 0 {
		return ""
	} else if len(slice) == 1 {
		return slice[0]
	}

	res := "{ "

	for i := range slice {
		res += " " + slice[i]
		if i < len(slice)-1 {
			res += ","
		}
	}

	res += " }"
	return res
}
