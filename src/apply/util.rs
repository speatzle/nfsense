use crate::definitions::object::{Address, AddressType};

pub fn convert_addresses_to_strings(addresses: Vec<Address>) -> Vec<String> {
    let mut list = vec![];
    for address in addresses {
        match address.address_type {
            AddressType::Host { address } => list.push(address.to_string()),
            AddressType::Range { range } => list.push(range.to_string()),
            AddressType::Network { network } => list.push(network.to_string()),
            AddressType::Group { .. } => {
                //TODO
            }
        }
    }
    list
}
