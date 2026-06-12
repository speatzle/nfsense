import type { SearchProvider, Fields, Field, Variants, Action } from "~/components/input/input";
import ArrayDisplay from "~/components/display/ArrayDisplay.vue";
import ElementDisplay from "~/components/display/ElementDisplay.vue";
import EnumTypeDisplay from "~/components/display/EnumTypeDisplay.vue";
import EnumValueDisplay from "~/components/display/EnumValueDisplay.vue";
import PortServiceDisplay from "~/components/display/PortServiceDisplay.vue";
import UpsertModal from "~/components/modals/UpsertModal.vue";
import IActionAdd from "~icons/material-symbols/add";

import { apiCall } from "./api";
import { useModals } from "./composables/modals";

const { pushModal } = useModals();

// --- Search Providers --- //
const GetEntity = (subsystem: string, entity: string): SearchProvider => {
  return async (_) => {
    const res = await apiCall(`${subsystem}.${entity}.list`, {});
    if (res.Error === null) {
      console.debug(entity, res.Data);
      return Object.fromEntries(res.Data.map((r: any) => [r.name, { display: r.name }]));
    } else {
      console.debug("error", res);
      return {};
    }
  };
};
const GetHardwareInterfaces = GetEntity("network", "links");
const GetInterfaces = GetEntity("network", "interfaces");
const GetAddresses = GetEntity("object", "addresses");
const GetVirtualRouters = GetEntity("network", "virtual_routers");
const GetServices = GetEntity("object", "services");
const GetPeers = GetEntity("vpn", "wireguard.peers");
const ICMPPacketTypes: SearchProvider = async (_) => {
  return {
    echo_reply: { display: "EchoReply" },
    destination_unreachable: { display: "DestinationUnreachable" },
    source_quench: { display: "SourceQuench" },
    redirect: { display: "Redirect" },
    echo_request: { display: "EchoRequest" },
    time_exceeded: { display: "TimeExceeded" },
    parameter_problem: { display: "ParameterProblem" },
    timestamp_request: { display: "TimestampRequest" },
    timestamp_reply: { display: "TimestampReply" },
    info_request: { display: "InfoRequest" },
    info_reply: { display: "InfoReply" },
    address_mask_request: { display: "AddressMaskRequest" },
    address_mask_reply: { display: "AddressMaskReply" },
    router_advertisement: { display: "RouterAdvertisement" },
    router_solicitation: { display: "RouterSolicitation" },
  };
};

// --- Definition Helpers ---
const c = {
  TextBox: (label) => ({ is: "TextBox", label }),
  MultilineTextBox: (label) => ({ is: "MultilineTextBox", label }),
  NumberBox: (label, props?: Record<string, unknown>) => ({ is: "NumberBox", label, props }),
  CheckBox: (label) => ({ is: "CheckBox", label }),
  Heading: (caption: string) => ({ is: "Heading", props: { caption } }),
  SingleSelect: (label, searchProvider: SearchProvider, actions?: Action[]) => ({
    is: "SingleSelect",
    label,
    props: { searchProvider },
    actions,
  }),
  MultiSelect: (label, searchProvider: SearchProvider, actions?: Action[]) => ({
    is: "MultiSelect",
    label,
    props: { searchProvider },
    actions,
  }),
  EnumInput: (label, variants: Variants) => ({ is: "EnumInput", label, props: { variants } }),
} satisfies { [k: string]: (label: string, ...args: any[]) => Field };
function toComposableFields<const T extends Record<string, unknown>>(fields: T) {
  return Object.fromEntries(Object.entries(fields).map(([k, v]) => [k, { [k]: v }])) as {
    [K in keyof T]: { [P in K]: T[K] };
  };
}

// oxfmt-ignore
const _f = {
  automatic_forward_rule: c.CheckBox("Automatic Forward Rule"),
  verdict: c.EnumInput("Verdict", {
    "accept": { display: "Accept" },
    "drop": { display: "Drop" },
    "continue": { display: "Continue" },
  }),
  interface: c.SingleSelect("Interface", GetInterfaces, [createAction("network", "interfaces", true)]),
  public_key: c.TextBox("Public Key"),
  members: c.MultiSelect("Members", GetInterfaces, [createAction("network", "interfaces")]),
};
const f = toComposableFields(_f);

function createAction(subsystem: string, entity: string, single?: boolean): Action {
  return {
    name: "Create",
    icon: IActionAdd,
    callback: async (key, $formData) => {
      const result = await pushModal(UpsertModal, { subsystem, entity });
      if (result) $formData[key] = single ? result : [...$formData[key], result];
    },
  };
}

// oxfmt-ignore
const fs = {
  rulesCommon: {
    source_addresses: c.MultiSelect("Source", GetAddresses, [createAction("object", "addresses")]),
    negate_source: c.CheckBox("Negate Source"),
    destination_addresses: c.MultiSelect("Destination", GetAddresses, [createAction("object", "addresses")]),
    negate_destination: c.CheckBox("Negate Destination"),
    services: c.MultiSelect("Services", GetServices, [createAction("object", "services")]),
    counter: c.CheckBox("Counter"),
    log: c.CheckBox("Log"),
  },
};

function withCommon(fields: Fields) {
  return {
    name: c.TextBox("Name"),
    ...fields,
    comment: c.MultilineTextBox("Comment"),
  } as Fields;
}

// oxfmt-ignore
const portVariants = {
  any: { display: "Any" },
  single: { display: "Single", fields: { port: c.NumberBox("Port") } },
  range: { display: "Range", fields: {
    start_port: c.NumberBox("Start Port"),
    end_port: c.NumberBox("End Port"),
  } },
};
const portFields = {
  source: c.EnumInput("Source", portVariants),
  destination: c.EnumInput("Destination", portVariants),
};

// --- Definitions --- //
type Subsystem = {
  name: string;
  entities: Record<string, Entity>;
};
export type Entity = {
  name: string;
  idType?: "Number" | "String";
  ordered?: true;
  fields?: Fields;
  default?: Record<string, any>;
  columns?: {
    // TODO: Merge into a SST
    heading: string;
    path: string;
    component?: Component;
    props?: any;
    componentProp?: string;
  }[];
};
// oxfmt-ignore
export const subsystems = {
  firewall: { name: "Firewall", entities: {
    forward_rules: {
      name: "Forward Rule",
      ordered: true,
      fields: withCommon({ ...fs.rulesCommon, ...f.verdict }),
      default: { verdict: "accept", counter: true },
      columns: [
        { heading: 'Name', path: 'name' },
        { heading: 'Source', path: 'source_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Destination', path: 'destination_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Service', path: 'services', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Verdict', path: 'verdict' },
        { heading: 'Counter', path: 'counter' },
        { heading: 'Log', path: 'log' },
        { heading: 'Comment', path: 'comment' },
      ]
    },
    destination_nat_rules: {
      name: "Destination NAT Rule",
      ordered: true,
      fields: withCommon({
        ...fs.rulesCommon,
        dnat_heading: c.Heading("DNAT"),
        dnat_address: c.SingleSelect("Destination", GetAddresses, [createAction("object", "addresses", true)]),
        dnat_service: c.SingleSelect("Service", GetServices, [createAction("object", "services", true)]),
        ...f.automatic_forward_rule,
      }),
      default: { counter: true },
      columns: [
        { heading: 'Name', path: 'name' },
        { heading: 'Source', path: 'source_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Destination', path: 'destination_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Service', path: 'services', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Translated Address', path: 'dnat_address', component: markRaw(ElementDisplay), componentProp: 'data' },
        { heading: 'Translated Service', path: 'dnat_service', component: markRaw(ElementDisplay), componentProp: 'data' },
        { heading: 'Counter', path: 'counter' },
        { heading: 'Comment', path: 'comment' },
      ]
    },
    source_nat_rules: {
      name: "Source NAT Rule",
      ordered: true,
      fields: withCommon({
        ...fs.rulesCommon,
        snat_heading: c.Heading("SNAT"),
        snat_type: c.EnumInput("Type", {
          "masquerade": { display: "Masquerade" },
          "snat": { display: "SNAT", fields: {
            address: c.SingleSelect("Source", GetAddresses, [createAction("object", "addresses", true)]),
            service: c.SingleSelect("Service", GetServices, [createAction("object", "services", true)]),
          } },
        }),
        ...f.automatic_forward_rule,
      }),
      default: { snat_type: "masquerade", counter: true },
      columns: [
        { heading: 'Name', path: 'name' },
        { heading: 'Source', path: 'source_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Destination', path: 'destination_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Service', path: 'services', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Type', path: 'snat_type', component: markRaw(EnumTypeDisplay), componentProp: 'data' },
        { heading: 'Translated Address', path: 'snat_type.snat.address', component: markRaw(ElementDisplay), componentProp: 'data' },
        { heading: 'Translated Service', path: 'snat_type.snat.service', component: markRaw(ElementDisplay), componentProp: 'data' },
        { heading: 'Counter', path: 'counter' },
        { heading: 'Comment', path: 'comment' },
      ],
    },
    inbound_rules: {
      name: "Inbound Rule",
      ordered: true,
      fields: withCommon({ ...fs.rulesCommon, ...f.verdict }),
      default: { verdict: "accept", counter: true },
      columns: [
        { heading: 'Name', path: 'name' },
        { heading: 'Source', path: 'source_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Service', path: 'services', component: markRaw(ArrayDisplay), componentProp: 'data' },
        { heading: 'Verdict', path: 'verdict' },
        { heading: 'Counter', path: 'counter' },
        { heading: 'Log', path: 'log' },
        { heading: 'Comment', path: 'comment' },
      ]
    },
  } },
  network: { name: "Network", entities: {
    interfaces: {
      name: "Interface",
      fields: withCommon({
        alias: c.TextBox("Alias"),
        interface_type: c.EnumInput("Type", {
          "hardware": { display: "Hardware", fields: { device: c.SingleSelect("Device", GetHardwareInterfaces) } },
          "vlan": { display: "VLAN", fields: {
            parent: c.SingleSelect("VLAN Parent", GetInterfaces, [createAction("network", "interfaces", true)]),
            id: c.NumberBox("VLAN ID", { min: 1, max: 4094 }),
          } },
          "bond": { display: "Bond", fields: { ...f.members } },
          "bridge": { display: "Bridge", fields: { ...f.members } },
        }),
        addressing_mode: c.EnumInput("Addressing Mode", {
          "none": { display: "None" },
          "static": { display: "Static", fields: { address: c.TextBox("Address") } },
          "dhcp": { display: "DHCP" },
        }),
        virtual_router: c.SingleSelect("Virtual Router", GetVirtualRouters, [createAction("network", "virtual_routers", true)]),
      }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Alias", path: "alias" },
        { heading: "Type", path: "interface_type", component: markRaw(EnumTypeDisplay), componentProp: "data" },
        { heading: "Addressing Mode", path: "addressing_mode", component: markRaw(EnumTypeDisplay), componentProp: "data" },
        { heading: "Address", path: "addressing_mode.static.address" },
        { heading: "Vlan Parent", path: "interface_type.vlan.parent" },
        { heading: "Vlan ID", path: "interface_type.vlan.id" },
        { heading: "Bond Members", path: "interface_type.bond.members", component: markRaw(ArrayDisplay), componentProp: "data" },
        { heading: "Bridge Members", path: "interface_type.bridge.members", component: markRaw(ArrayDisplay), componentProp: "data" },
        { heading: "Virtual Router", path: "virtual_router" },
        { heading: "Comment", path: "comment" },
      ],
    },
    policy_routes: {
      name: "Policy Route",
      ordered: true,
      columns: [
        { heading: 'Name', path: 'name' },
        { heading: 'Interface', path: 'interface' },
        { heading: 'Gateway', path: 'gateway' },
        { heading: 'Destination', path: 'destination' },
        { heading: 'Metric', path: 'metric' },
      ],
    },
    static_routes: {
      name: "Static Route",
      fields: withCommon({
        ...f.interface,
        gateway: c.SingleSelect("Gateway", GetAddresses, [createAction("object", "addresses", true)]),
        destination: c.TextBox("Destination"),
        metric: c.NumberBox("Metric"),
      }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Interface", path: "interface" },
        { heading: "Gateway", path: "gateway" },
        { heading: "Destination", path: "destination" },
        { heading: "Metric", path: "metric" },
      ],
    },
    virtual_routers: {
      name: "Virtual Routers",
      fields: withCommon({ table_id: c.NumberBox("Table ID") }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Table ID", path: "table_id" },
        { heading: "Comment", path: "comment" },
      ],
    },
  } },
  object: { name: "Object", entities: {
    addresses: {
      name: "Address",
      fields: withCommon({
        address_type: c.EnumInput("Type", {
          "host": { display: "Host", fields: { address: c.TextBox("Address") } },
          "range": { display: "Range", fields: { range: c.TextBox("Range") } },
          "network": { display: "Network", fields: { network: c.TextBox("Network") } },
          "group": { display: "Group", fields: { members: c.MultiSelect("Members", GetAddresses, [createAction("object", "addresses")]) } },
        }),
      }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Type", path: "address_type", component: markRaw(EnumTypeDisplay), componentProp: "data" },
        { heading: "Value", path: "address_type", component: markRaw(EnumValueDisplay), componentProp: "data", props: { definition: {
          host: { path: "host.address", component: undefined },
          range: { path: "range.address", component: undefined },
          network: { path: "network.network", component: undefined },
          group: { path: "group.members", component: ArrayDisplay },
        } } },
        { heading: "Comment", path: "comment" },
      ],
    },
    services: {
      name: "Service",
      fields: withCommon({
        service_type: c.EnumInput("Type", {
          "tcp": { display: "TCP", fields: portFields },
          "udp": { display: "UDP", fields: portFields },
          "icmp": { display: "ICMP", fields: { ptypes: c.MultiSelect("Packet Types", ICMPPacketTypes) } },
          "group": { display: "Group", fields: { members: c.MultiSelect("Members", GetServices, [createAction("object", "services")]) } },
        }),
      }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Type", path: "service_type", component: markRaw(EnumTypeDisplay), componentProp: "data", props: { definition: {
            tcp: { path: "tcp", component: PortServiceDisplay },
            udp: { path: "udp", component: PortServiceDisplay },
            icmp: { path: "icmp.ptypes", component: undefined },
            group: { path: "group.members", component: ArrayDisplay },
        } } },
        { heading: "Value", path: "service_type", component: markRaw(EnumValueDisplay), componentProp: "data", props: { definition: {
            tcp: { path: "tcp", component: PortServiceDisplay },
            udp: { path: "udp", component: PortServiceDisplay },
            icmp: { path: "icmp.ptypes", component: undefined },
            group: { path: "group.members", component: ArrayDisplay },
        } } },
        { heading: "Comment", path: "comment" },
      ],
    },
  } },
  service: { name: "Service", entities: {
    dhcp_servers: {
      name: "DHCP Server",
      fields: withCommon({
        ...f.interface,
        pool: c.MultiSelect("Pool", GetAddresses, [createAction("object", "addresses")]),
        gateway_mode: c.EnumInput("Gateway Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { gateway: c.SingleSelect("Gateway", GetAddresses, [createAction("object", "addresses", true)]) } },
        }),
        dns_server_mode: c.EnumInput("DNS Server Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { dns_servers: c.MultiSelect("DNS Servers", GetAddresses, [createAction("object", "addresses")]) } },
        }),
        ntp_server_mode: c.EnumInput("NTP Server Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { ntp_servers: c.MultiSelect("NTP Servers", GetAddresses, [createAction("object", "addresses")]) } },
        }),
        lease_time: c.NumberBox("Lease Time"),
      }),
      columns: [
        { heading: "Interface", path: "interface" },
        { heading: "Pool", path: "pool", component: markRaw(ArrayDisplay), componentProp: "data" },
        { heading: "Comment", path: "comment" },
      ],
    },
    dns_servers: {
      name: "DNS Server",
      fields: withCommon({ ...f.interface }),
      columns: [
        { heading: "Interface", path: "interface" },
        { heading: "Comment", path: "comment" },
      ],
    },
    ntp_servers: {
      name: "NTP Server",
      fields: withCommon({ ...f.interface }),
      columns: [
        { heading: "Interface", path: "interface" },
        { heading: "Comment", path: "comment" },
      ],
    },
  } },
  system: { name: "System", entities: {
    users: {
      name: "User",
      fields: withCommon({ password: c.TextBox("Password") }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Comment", path: "comment" },
      ],
    },
  } },
  vpn: { name: "VPN", entities: {
    "wireguard.interfaces": {
      name: "Wireguard Interface",
      fields: withCommon({
        ...f.public_key,
        private_key: c.TextBox("Private Key"),
        listen_port: c.NumberBox("Listen Port"),
        peers: c.MultiSelect("Peers", GetPeers, [createAction("vpn", "wireguard.peers")]),
      }),
    },
    "wireguard.peers": {
      name: "Wireguard Peer",
      fields: withCommon({
        ...f.public_key,
        preshared_key: c.TextBox("Preshared Key"),
        allowed_ips: c.MultiSelect("Allowed IPs", GetAddresses, [createAction("object", "addresses")]),
        endpoint: c.TextBox("Endpoint"),
        persistent_keepalive: c.NumberBox("Persistent Keepalive"),
      }),
      columns: [
        { heading: "Name", path: "name" },
        { heading: "Allowed IPs", path: "allowed_ips", component: markRaw(ArrayDisplay), componentProp: "data" },
        { heading: "Endpoint", path: "endpoint" },
        { heading: "Persistent Keepalive", path: "persistent_keepalive" },
        { heading: "Comment", path: "comment" },
      ],
    },
  } },
} as const satisfies Record<string, Subsystem>;
