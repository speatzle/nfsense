package util

import (
	"nfsense.net/nfsense/internal/definitions/object"
)

// ResolveBaseAddresses Resolves all groups to their base Addresses
func ResolveBaseAddresses(allAddresses map[string]object.Address, addressNames []string) []object.Address {
	baseAddresses := []object.Address{}

	for _, addressName := range addressNames {
		address := allAddresses[addressName]

		if address.Type == object.AddressGroup {
			baseAddresses = append(baseAddresses, resolveAddressChildren(allAddresses, address)...)
		} else {
			baseAddresses = append(baseAddresses, address)
		}

	}

	return baseAddresses
}

func resolveAddressChildren(allAddresses map[string]object.Address, a object.Address) []object.Address {
	addressList := []object.Address{}
	for _, addressName := range *a.Children {
		address := allAddresses[addressName]

		if address.Type == object.AddressGroup {
			addressList = append(addressList, resolveAddressChildren(allAddresses, address)...)
		} else {
			addressList = append(addressList, address)
		}
	}
	return addressList
}
