import type { SearchProvider, Fields, Field, Variants } from "~/components/input/input";
import { apiCall } from "./api";

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
  SingleSelect: (label, searchProvider: SearchProvider) => ({
    is: "SingleSelect",
    label,
    props: { searchProvider },
  }),
  MultiSelect: (label, searchProvider: SearchProvider) => ({
    is: "MultiSelect",
    label,
    props: { searchProvider },
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
  interface: c.SingleSelect("Interface", GetInterfaces),
  public_key: c.TextBox("Public Key"),
  members: c.MultiSelect("Members", GetInterfaces),
};
const f = toComposableFields(_f);

// oxfmt-ignore
const fs = {
  rulesCommon: {
    source_addresses: c.MultiSelect("Source", GetAddresses),
    negate_source: c.CheckBox("Negate Source"),
    destination_addresses: c.MultiSelect("Destination", GetAddresses),
    negate_destination: c.CheckBox("Negate Destination"),
    services: c.MultiSelect("Services", GetServices),
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
  fields?: Fields;
  default?: Record<string, any>;
  columns?: {
    // TODO: Merge into a SST
    heading: string;
    path: string;
    component: Component;
    props: any;
  }[];
};
// oxfmt-ignore
export const subsystems = {
  firewall: { name: "Firewall", entities: {
    forward_rules: {
      name: "Forward Rule",
      idType: "Number",
      fields: withCommon({ ...fs.rulesCommon, ...f.verdict }),
      default: { verdict: "accept", counter: true },
    },
    destination_nat_rules: {
      name: "Destination NAT Rule",
      idType: "Number",
      fields: withCommon({
        ...fs.rulesCommon,
        dnat_heading: c.Heading("DNAT"),
        dnat_address: c.SingleSelect("Destination", GetAddresses),
        dnat_service: c.SingleSelect("Service", GetServices),
        ...f.automatic_forward_rule,
      }),
      default: { counter: true },
    },
    source_nat_rules: {
      name: "Source NAT Rule",
      idType: "Number",
      fields: withCommon({
        ...fs.rulesCommon,
        snat_heading: c.Heading("SNAT"),
        snat_type: c.EnumInput("Type", {
          "masquerade": { display: "Masquerade" },
          "snat": { display: "SNAT", fields: {
            address: c.SingleSelect("Source", GetAddresses),
            service: c.SingleSelect("Service", GetServices),
          } },
        }),
        ...f.automatic_forward_rule,
      }),
      default: { snat_type: "masquerade", counter: true },
    },
    inbound_rules: {
      name: "Inbound Rule",
      idType: "Number",
      fields: withCommon({ ...fs.rulesCommon, ...f.verdict }),
      default: { verdict: "accept", counter: true },
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
            parent: c.SingleSelect("VLAN Parent", GetInterfaces),
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
        virtual_router: c.SingleSelect("Virtual Router", GetVirtualRouters),
      }),
    },
    static_routes: {
      name: "Static Route",
      idType: "Number",
      fields: withCommon({
        ...f.interface,
        gateway: c.TextBox("Gateway"),
        destination: c.TextBox("Destination"),
        metric: c.NumberBox("Metric"),
      }),
    },
    virtual_routers: {
      name: "Virtual Routers",
      idType: "Number",
      fields: withCommon({ table_id: c.NumberBox("Table ID") }),
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
          "group": { display: "Group", fields: { members: c.MultiSelect("Members", GetAddresses) } },
        }),
      }),
    },
    services: {
      name: "Service",
      fields: withCommon({
        service_type: c.EnumInput("Type", {
          "tcp": { display: "TCP", fields: portFields },
          "udp": { display: "UDP", fields: portFields },
          "icmp": { display: "ICMP", fields: { ptypes: c.MultiSelect("Packet Types", ICMPPacketTypes) } },
          "group": { display: "Group", fields: { members: c.MultiSelect("Members", GetServices) } },
        }),
      }),
    },
  } },
  service: { name: "Service", entities: {
    dhcp_servers: {
      name: "DHCP Server",
      idType: "Number",
      fields: withCommon({
        ...f.interface,
        pool: c.MultiSelect("Pool", GetAddresses),
        gateway_mode: c.EnumInput("Gateway Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { gateway: c.SingleSelect("Gateway", GetAddresses) } },
        }),
        dns_server_mode: c.EnumInput("DNS Server Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { dns_servers: c.MultiSelect("DNS Servers", GetAddresses) } },
        }),
        ntp_server_mode: c.EnumInput("NTP Server Mode", {
          "none": { display: "None" },
          "interface": { display: "Interface" },
          "specify": { display: "Specify", fields: { ntp_servers: c.MultiSelect("NTP Servers", GetAddresses) } },
        }),
        lease_time: c.NumberBox("Lease Time"),
      }),
    },
    ntp_servers: {
      name: "NTP Server",
      idType: "Number",
      fields: withCommon({ ...f.interface }),
    },
    dns_servers: {
      name: "DNS Server",
      idType: "Number",
      fields: withCommon({ ...f.interface }),
    },
  } },
  vpn: { name: "VPN", entities: {
    "wireguard.interfaces": {
      name: "Wireguard Interface",
      fields: withCommon({
        ...f.public_key,
        private_key: c.TextBox("Private Key"),
        listen_port: c.NumberBox("Listen Port"),
        peers: c.MultiSelect("Peers", GetPeers),
      }),
    },
    "wireguard.peers": {
      name: "Wireguard Peer",
      fields: withCommon({
        ...f.public_key,
        preshared_key: c.TextBox("Preshared Key"),
        allowed_ips: c.MultiSelect("Allowed IPs", GetAddresses),
        endpoint: c.TextBox("Endpoint"),
        persistent_keepalive: c.NumberBox("Persistent Keepalive"),
      }),
    },
  } },
  system: { name: "System", entities: {
    users: {
      name: "User",
      fields: withCommon({ password: c.TextBox("Password") }),
    },
  } },
} as const satisfies Record<string, Subsystem>;
