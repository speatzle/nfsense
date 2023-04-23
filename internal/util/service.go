package util

import "nfsense.net/nfsense/internal/definitions/object"

// ResolveBaseServices Resolves all groups to their base Services
func ResolveBaseServices(allServices map[string]object.Service, serviceNames []string) []object.Service {
	baseServices := []object.Service{}

	for _, serviceName := range serviceNames {
		service := allServices[serviceName]

		if service.Type == object.ServiceGroup {
			baseServices = append(baseServices, resolveServiceChildren(allServices, service)...)
		} else {
			baseServices = append(baseServices, service)
		}

	}

	return baseServices
}

func resolveServiceChildren(allServices map[string]object.Service, s object.Service) []object.Service {
	serviceList := []object.Service{}
	for _, serviceName := range *s.Children {
		service := allServices[serviceName]

		if service.Type == object.ServiceGroup {
			serviceList = append(serviceList, resolveServiceChildren(allServices, service)...)
		} else {
			serviceList = append(serviceList, service)
		}
	}
	return serviceList
}
