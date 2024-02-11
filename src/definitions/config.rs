use serde::{Deserialize, Serialize};
use validator::Validate;

use super::firewall;
use super::firewall::SNATType;
use super::network;
use super::network::AddressingMode;
use super::network::NetworkInterfaceType;
use super::object;
use super::object::AddressType;
use super::object::ServiceType;
use super::service;
use super::service::DNSServerMode;
use super::service::GatewayMode;
use super::service::NTPServerMode;
use super::system;
use super::vpn;
use crate::macro_db;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Config {
    pub config_version: u64,
    pub network: network::Network,
    pub object: object::Object,
    pub system: system::System,
    pub service: service::Service,
    pub vpn: vpn::VPN,
    pub firewall: firewall::Firewall,
}

macro_db!(
    {
        // ForwardRule
        [ M: source_addresses, firewall::ForwardRule, object.addresses; firewall.forward_rules ()],
        [ M: destination_addresses, firewall::ForwardRule, object.addresses; firewall.forward_rules ()],

        // DestinationNATRule
        [ M: source_addresses, firewall::DestinationNATRule, object.addresses; firewall.destination_nat_rules ()],
        [ M: destination_addresses, firewall::DestinationNATRule, object.addresses; firewall.destination_nat_rules ()],
        [ O: dnat_address, firewall::DestinationNATRule, object.addresses; firewall.destination_nat_rules ()],

        // SourceNATRule
        [ M: source_addresses, firewall::SourceNATRule, object.addresses; firewall.source_nat_rules ()],
        [ M: destination_addresses, firewall::SourceNATRule, object.addresses; firewall.source_nat_rules ()],
        [ EO: address, firewall::SourceNATRule, object.addresses; firewall.source_nat_rules (snat_type, SNATType, SNAT, address)],

        // StaticRoutes
        [ S: gateway, network::StaticRoute, object.addresses; network.static_routes ()],
        [ S: destination, network::StaticRoute, object.addresses; network.static_routes ()],

        // NetworkInteface
        [ E: address, network::NetworkInterface, object.addresses; network.interfaces (addressing_mode, AddressingMode, Static, address)],

        // Address
        [ EM: members, object::Address, object.addresses; object.addresses (address_type, AddressType, Group, members)],

        // DHCPServer
        [ M: pool, service::DHCPServer, object.addresses; service.dhcp_servers ()],
        [ E: gateway, service::DHCPServer, object.addresses; service.dhcp_servers (gateway_mode, GatewayMode, Specify, gateway)],
        [ EM: dns_servers, service::DHCPServer, object.addresses; service.dhcp_servers (dns_server_mode, DNSServerMode, Specify, dns_servers)],
        [ EM: ntp_servers, service::DHCPServer, object.addresses; service.dhcp_servers (ntp_server_mode, NTPServerMode, Specify, ntp_servers)],

        // WireguardPeer
        [ M: allowed_ips, vpn::WireguardPeer, object.addresses; vpn.wireguard.peers ()],
        [ O: endpoint, vpn::WireguardPeer, object.addresses; vpn.wireguard.peers ()],

        ->
        object::Address
    },
    {
        // ForwardRule
        [ M: services, firewall::ForwardRule, object.services; firewall.forward_rules ()],

        // DestinationNATRule
        [ M: services, firewall::DestinationNATRule, object.services; firewall.destination_nat_rules ()],
        [ O: dnat_service, firewall::DestinationNATRule, object.services; firewall.destination_nat_rules ()],

        // SourceNATRule
        [ M: services, firewall::SourceNATRule, object.services; firewall.source_nat_rules ()],
        [ EO: service, firewall::SourceNATRule, object.services; firewall.source_nat_rules (snat_type, SNATType, SNAT, service)],
        [ EM: members, object::Service, object.services; object.services (service_type, ServiceType, Group, members)],
        ->
        object::Service
    },
    {
        // StaticRoute
        [ S: interface, network::StaticRoute, network.interfaces; network.static_routes ()],

        // DHCPServer
        [ S: interface, service::DHCPServer, network.interfaces; service.dhcp_servers ()],

        // DNSServer
        [ S: interface, service::DNSServer, network.interfaces; service.dns_servers ()],

        // NTPServer
        [ S: interface, service::NTPServer, network.interfaces; service.ntp_servers ()],

        // NetworkInterface
        [ E: parent, network::NetworkInterface, network.interfaces; network.interfaces (interface_type, NetworkInterfaceType, Vlan, vlan_parent)],
        [ EM: members, network::NetworkInterface, network.interfaces; network.interfaces (interface_type, NetworkInterfaceType, Bond, bond_members)],
        [ EM: members, network::NetworkInterface, network.interfaces; network.interfaces (interface_type, NetworkInterfaceType, Bridge, bridge_members)],
        ->
        network::NetworkInterface
    },
    {
        // WireguardInterface
        [ M: peers, vpn::WireguardInterface, vpn.wireguard.peers; vpn.wireguard.interfaces ()],
        ->
        vpn::WireguardPeer
    },
);
