import { toFormValidator } from '@vee-validate/zod';
import * as zod from 'zod';
import { SearchProvider, Options } from '~/components/inputs/DropdownInput.vue';
import { apiCall } from './api';

const GetHardwareInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.get_links', {});
  if (res.Error === null) {
    console.debug('links', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.get_interfaces', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    let obj = {} as Options;
    Object.keys(res.Data).forEach(function (key, index) {
      obj[key] = {
        display: key,
      };
    });
    return obj;
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetAddresses: SearchProvider = async (o) => {
  let res = await apiCall('object.get_addresses', {});
  if (res.Error === null) {
    console.debug('addresses', res.Data);
    let obj = {} as Options;
    Object.keys(res.Data).forEach(function (key, index) {
      obj[key] = {
        display: key,
      };
    });
    return obj;
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetServices: SearchProvider = async (o) => {
  let res = await apiCall('Object.get_services', {});
  if (res.Error === null) {
    console.debug('services', res.Data);
    let obj = {} as Options;
    Object.keys(res.Data).forEach(function (key, index) {
      obj[key] = {
        display: key,
      };
    });
    return obj;
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetPeers: SearchProvider = async (o) => {
  let res = await apiCall('VPN.get_wireguard_peers', {});
  if (res.Error === null) {
    console.debug('peers', res.Data);
    let obj = {} as Options;
    Object.keys(res.Data).forEach(function (key, index) {
      obj[key] = {
        display: key,
      };
    });
    return obj;
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

export const editTypes: { [key: string]: { [key: string]: any } } = {
  'firewall': {
    name: 'firewall',
    'forwardrules': {
      name: 'forward_rule',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
          verdict: zod.string(),
          counter: zod.boolean(),
          comment: zod.string().optional(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox' },
            { key: 'match.source_addresses', label: 'Source', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.destination_addresses', label: 'Destination', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.services', label: 'Services', as: 'MultiSelect', props: { searchProvider: GetServices } },
            { key: 'verdict', label: 'Verdict', as: 'PillBar', props: { options: { accept: { display: 'Accept' }, drop: { display: 'Drop' }, continue: { display: 'Continue' } } } },
            { key: 'counter', label: 'Counter', as: 'CheckBox' },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'destinationnatrules': {
      name: 'destination_nat_rule',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
          verdict: zod.string(),
          counter: zod.boolean(),
          comment: zod.string().optional(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox' },
            { key: 'match.source_addresses', label: 'Source', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.destination_addresses', label: 'Destination', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.services', label: 'Services', as: 'MultiSelect', props: { searchProvider: GetServices } },
          ],
        },
        {
          title: 'DNAT',
          fields: [
            { key: 'address', label: 'Destination', as: 'SingleSelect', props: { searchProvider: GetAddresses } },
            { key: 'service', label: 'Service', as: 'SingleSelect', props: { searchProvider: GetServices } },
            { key: 'counter', label: 'Counter', as: 'CheckBox' },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'sourcenatrules': {
      name: 'source_nat_rule',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
          verdict: zod.string(),
          counter: zod.boolean(),
          comment: zod.string().optional(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox' },
            { key: 'match.source_addresses', label: 'Source', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.destination_addresses', label: 'Destination', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'match.services', label: 'Services', as: 'MultiSelect', props: { searchProvider: GetServices } },
          ],
        },
        {
          title: 'SNAT',
          fields: [
            { key: 'type', label: 'Type', as: 'PillBar', props: { options: { snat: { display: 'SNAT' }, masquerade: { display: 'Masquerade' } } } },
            { key: 'address', label: 'Source', as: 'SingleSelect', enabled: (values: any) => (values['type'] == 'snat'), props: { searchProvider: GetAddresses } },
            { key: 'service', label: 'Service', as: 'SingleSelect', enabled: (values: any) => (values['type'] == 'snat'), props: { searchProvider: GetServices } },
            { key: 'counter', label: 'Counter', as: 'CheckBox' },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
  },
  'network': {
    name: 'network',
    'interfaces': {
      name: 'interface',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
          type: zod.string(),
          hardware_interface: zod.string().optional(),
          vlan_id: zod.number().optional(),
          comment: zod.string().optional(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox', default: 'placeholder' },
            { key: 'type', label: 'Type', as: 'PillBar', props: { options: { hardware: { display: 'Hardware' }, vlan: { display: 'VLAN' }, bond: { display: 'Bond' }, bridge: { display: 'Bridge' } } } },
            { key: 'hardware_device', label: 'Hardware Device', as: 'SingleSelect', enabled: (values: any) => (values['type'] == 'hardware'), props: { searchProvider: GetHardwareInterfaces } },
            { key: 'vlan_parent', label: 'VLAN Parent', as: 'SingleSelect', enabled: (values: any) => (values['type'] == 'vlan'), props: { searchProvider: GetInterfaces } },
            { key: 'vlan_id', label: 'VLAN ID', as: 'NumberBox', props: { min: 1, max: 4094 }, enabled: (values: any) => (values['type'] == 'vlan') },
            { key: 'bond_members', label: 'Bond Members', as: 'MultiSelect', enabled: (values: any) => (values['type'] == 'bond'), props: { searchProvider: GetHardwareInterfaces } },
            { key: 'bridge_members', label: 'Bridge Members', as: 'MultiSelect', enabled: (values: any) => (values['type'] == 'bridge'), props: { searchProvider: GetHardwareInterfaces } },
          ],
        },
        {
          title: 'Addressing',
          fields: [
            { key: 'addressing_mode', label: 'Addressing Mode', as: 'PillBar', props: { options: { none: { display: 'None' }, static: { display: 'Static' }, dhcp: { display: 'DHCP' } } } },
            { key: 'address', label: 'Address', as: 'TextBox', enabled: (values: any) => (values['addressing_mode'] == 'static') },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'staticroutes': {
      name: 'static_route',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox' },
            { key: 'interface', label: 'Interface', as: 'SingleSelect', props: { searchProvider: GetInterfaces } },
            { key: 'gateway', label: 'Gateway', as: 'TextBox' },
            { key: 'destination', label: 'Destination', as: 'TextBox' },
            { key: 'metric', label: 'Metric', as: 'NumberBox' },
          ],
        },
      ],
    },
  },
  'object': {
    name: 'object',
    'addresses': {
      name: 'address',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox', default: 'placeholder' },
            { key: 'type', label: 'Type', as: 'PillBar', props: { options: { host: { display: 'Host' }, range: { display: 'Range' }, network: { display: 'Network' }, group: { display: 'Group' } } } },
            { key: 'host', label: 'Host', as: 'TextBox', default: 'placeholder', enabled: (values: any) => (values['type'] == 'host') },
            { key: 'range', label: 'Range', as: 'TextBox', default: 'placeholder', enabled: (values: any) => (values['type'] == 'range') },
            { key: 'network', label: 'Network', as: 'TextBox', default: 'placeholder', enabled: (values: any) => (values['type'] == 'network') },
            { key: 'children', label: 'Children', as: 'MultiSelect', enabled: (values: any) => (values['type'] == 'group'), props: { searchProvider: GetAddresses } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'services': {
      name: 'service',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox', default: 'placeholder' },
            { key: 'type', label: 'Type', as: 'PillBar', props: { options: { tcp: { display: 'TCP' }, udp: { display: 'UDP' }, icmp: { display: 'ICMP' }, group: { display: 'Group' } } } },
            { key: 'sport_start', label: 'Source Port Start', as: 'NumberBox', enabled: (values: any) => (values['type'] == 'tcp' || values['type'] == 'udp') },
            { key: 'sport_end', label: 'Source Port End', as: 'NumberBox', enabled: (values: any) => (values['type'] == 'tcp' || values['type'] == 'udp') },
            { key: 'dport_start', label: 'Destination Port Start', as: 'NumberBox', enabled: (values: any) => (values['type'] == 'tcp' || values['type'] == 'udp') },
            { key: 'dport_end', label: 'Destination Port End', as: 'NumberBox', enabled: (values: any) => (values['type'] == 'tcp' || values['type'] == 'udp') },
            { key: 'icmp_code', label: 'ICMP Code', as: 'NumberBox', enabled: (values: any) => (values['type'] == 'icmp') },
            { key: 'children', label: 'Children', as: 'MultiSelect', enabled: (values: any) => (values['type'] == 'group'), props: { searchProvider: GetServices } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
  },
  'service': {
    name: 'service',
    'dhcpservers': {
      name: 'dhcp_server',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'interface', label: 'Interface', as: 'SingleSelect', props: { searchProvider: GetInterfaces } },
            { key: 'pool', label: 'Pool', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'gateway_mode', label: 'Gateway Mode', as: 'PillBar', props: { options: { none: { display: 'None' }, interface: { display: 'Interface' }, specify: { display: 'Specify' } } } },
            { key: 'gateway', label: 'Gateway', as: 'SingleSelect', enabled: (values: any) => (values['gateway_mode'] == 'specify'), props: { searchProvider: GetAddresses } },
            { key: 'dns_server_mode', label: 'DNS Server Mode', as: 'PillBar', props: { options: { none: { display: 'None' }, interface: { display: 'Interface' }, specify: { display: 'Specify' } } } },
            { key: 'dns_servers', label: 'DNS Servers', as: 'MultiSelect', enabled: (values: any) => (values['dns_server_mode'] == 'specify'), props: { searchProvider: GetAddresses } },
            { key: 'ntp_server_mode', label: 'NTP Server Mode', as: 'PillBar', props: { options: { none: { display: 'None' }, interface: { display: 'Interface' }, specify: { display: 'Specify' } } } },
            { key: 'ntp_servers', label: 'NTP Servers', as: 'MultiSelect', enabled: (values: any) => (values['ntp_server_mode'] == 'specify'), props: { searchProvider: GetAddresses } },
            { key: 'default_lease_time', label: 'Default Lease Time', as: 'NumberBox' },
            { key: 'max_lease_time', label: 'Max Lease Time', as: 'NumberBox' },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'ntpservers': {
      name: 'ntp_server',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'interface', label: 'Interface', as: 'SingleSelect', props: { searchProvider: GetInterfaces } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'dnsservers': {
      name: 'dns_server',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'interface', label: 'Interface', as: 'SingleSelect', props: { searchProvider: GetInterfaces } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
  },
  'vpn': {
    name: 'vpn',
    'wireguardinterfaces': {
      name: 'wireguard_interface',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox', default: 'placeholder' },
            { key: 'public_key', label: 'Public Key', as: 'TextBox', default: 'placeholder' },
            { key: 'private_key', label: 'Private Key', as: 'TextBox', default: 'placeholder' },
            { key: 'listen_port', label: 'Listen Port', as: 'NumberBox' },
            { key: 'peers', label: 'Peers', as: 'MultiSelect', props: { searchProvider: GetPeers } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
    'wireguardpeers': {
      name: 'wireguard_peer',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox', default: 'placeholder' },
            { key: 'public_key', label: 'Public Key', as: 'TextBox', default: 'placeholder' },
            { key: 'preshared_key', label: 'Preshared Key', as: 'TextBox', default: 'placeholder' },
            { key: 'allowed_ips', label: 'Allowed IPs', as: 'MultiSelect', props: { searchProvider: GetAddresses } },
            { key: 'endpoint', label: 'Endpoint', as: 'TextBox', default: 'placeholder' },
            { key: 'persistent_keepalive', label: 'Persistent Keepalive', as: 'NumberBox' },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
  },
  'system': {
    name: 'system',
    'users': {
      name: 'user',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      sections: [
        {
          fields: [
            { key: 'name', label: 'Name', as: 'TextBox' },
            { key: 'password', label: 'Password', as: 'TextBox', props: { type: 'password' } },
            { key: 'comment', label: 'Comment', as: 'MultilineTextBox' },
          ],
        },
      ],
    },
  },
};