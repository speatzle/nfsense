package util

import "github.con/speatzle/nfsense/pkg/definitions"

// ResolveBaseAddresses Resolves all groups to their base Addresses
func ResolveBaseAddresses(allAddresses map[string]definitions.Address, addressNames []string) []definitions.Address {
	baseAddresses := []definitions.Address{}

	for _, addressName := range addressNames {
		address := allAddresses[addressName]

		if address.Type == definitions.AddressGroup {
			baseAddresses = append(baseAddresses, resolveAddressChildren(allAddresses, address)...)
		} else {
			baseAddresses = append(baseAddresses, address)
		}

	}

	return baseAddresses
}

func resolveAddressChildren(allAddresses map[string]definitions.Address, a definitions.Address) []definitions.Address {
	addressList := []definitions.Address{}
	for _, addressName := range *a.Children {
		address := allAddresses[addressName]

		if address.Type == definitions.AddressGroup {
			addressList = append(addressList, resolveAddressChildren(allAddresses, address)...)
		} else {
			addressList = append(addressList, address)
		}
	}
	return addressList
}
