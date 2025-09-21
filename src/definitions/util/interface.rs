use crate::definitions::{
    network::AddressingMode::Static as StaticAddressingMode,
    object::{Address, AddressType},
};
use crate::state::RpcState;

pub fn generate_interface_address_objects(state: &RpcState) -> Vec<Address> {
    state
        .config_manager
        .get_pending_config()
        .network
        .interfaces
        .iter()
        .flat_map(|interface| {
            if let StaticAddressingMode { address } = interface.addressing_mode {
                // Strip the ip out of the network
                let network = ipnet::IpNet::new(address.network(), address.prefix_len()).unwrap();
                vec![
                    Address {
                        name: format!("interface_{}_address", interface.name),
                        comment: "Auto-generated Interface Address".to_owned(),
                        address_type: AddressType::Host {
                            address: address.addr(),
                        },
                    },
                    Address {
                        name: format!("interface_{}_network", interface.name),
                        comment: "Auto-generated Interface Network".to_owned(),
                        address_type: AddressType::Network { network: network },
                    },
                ]
            } else {
                Vec::new()
            }
        })
        .collect()
}
