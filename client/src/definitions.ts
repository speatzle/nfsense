import { toFormValidator } from '@vee-validate/zod';
import * as zod from 'zod';
import { SearchProvider, Options } from '~/components/inputs/DropdownInput.vue';
import { apiCall } from './api';

const GetHardwareInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.links.list', {});
  if (res.Error === null) {
    console.debug('links', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.interfaces.list', {});
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
  let res = await apiCall('object.addresses.list', {});
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
  let res = await apiCall('Object.services.list', {});
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
  let res = await apiCall('VPN.wireguard.peers.list', {});
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

const PortDefinition: Object = {
  'any': { display: 'Any'},
  'single': {
    display: 'Single',
    fields: {
      port: { is: 'NumberBox', label: 'Port'},
    },
  },
  'range': {
    display: 'Range',
    fields: {
      start_port: { is: 'NumberBox', label: 'Start Port'},
      end_port: { is: 'NumberBox', label: 'End Port'},
    },
  },
};

export const editTypes: { [key: string]: { [key: string]: any } } = {
  'firewall': {
    name: 'Firewall',
    'forward_rules': {
      name: 'Forward Rule',
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
    'destination_nat_rules': {
      name: 'Destination NAT Rule',
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
    'source_nat_rules': {
      name: 'Source NAT Rule',
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
    name: 'Network',
    'interfaces': {
      name: 'Interface',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        alias: { is: 'TextBox', label: 'Alias'},
        interface_type: { is: 'EnumInput', label: 'Type', props: { variants: {
          'hardware': {
            display: 'Hardware',
            fields: {
              device: { is: 'SingleSelect', label: 'Device', props: { searchProvider: GetHardwareInterfaces }},
            },
          },
          'vlan': {
            display: 'VLAN',
            fields: {
              vlan_parent: { is: 'SingleSelect', label: 'VLAN Parent', props: { searchProvider: GetInterfaces}},
              vlan_id: { is: 'NumberBox', label: 'VLAN ID', props: { min: 1, max: 4094 }},
            },
          },
          'bond': {
            display: 'Bond',
            fields: {
              members: { is: 'MultiSelect', label: 'Members', props: { searchProvider: GetInterfaces}},
            },
          },
          'bridge': {
            display: 'Bridge',
            fields: {
              members: { is: 'MultiSelect', label: 'Members', props: { searchProvider: GetInterfaces}},
            },
          },
        }}},
        addressing_mode: { is: 'EnumInput', label: 'Addressing Mode', props: { variants: {
          'none': { display: 'None' },
          'static': {
            display: 'Static',
            fields: {
              address: { is: 'TextBox', label: 'Address'},
            },
          },
          'dhcp': { display: 'DHCP' },
        }}},
        comment: { is: 'TextBox', label: 'Comment'},
      },
    },
    'static_routes': {
      name: 'Static Route',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
        }),
      ),
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        gatway: { is: 'TextBox', label: 'Gateway'},
        destination: { is: 'TextBox', label: 'Destination'},
        metric: { is: 'NumberBox', label: 'Metric'},
      },
    },
  },
  'object': {
    name: 'object',
    'addresses': {
      name: 'Address',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        address_type: { is: 'EnumInput', label: 'Type', props: { variants: {
          'host': {
            display: 'Host',
            fields: {
              address: { is: 'TextBox', label: 'Address'},
            },
          },
          'range': {
            display: 'Range',
            fields: {
              range: { is: 'TextBox', label: 'Range'},
            },
          },
          'network': {
            display: 'Network',
            fields: {
              network: { is: 'TextBox', label: 'Network'},
            },
          },
          'group': {
            display: 'Group',
            fields: {
              members: { is: 'MultiSelect', label: 'Members', props: { searchProvider: GetAddresses}},
            },
          },
        }}},
        comment: { is: 'TextBox', label: 'Comment'},
      },
    },
    'services': {
      name: 'Service',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        service_type: { is: 'EnumInput', label: 'Type', props: { variants: {
          'tcp': {
            display: 'TCP',
            fields: {
              source: { is: 'EnumInput', label: 'Source', props: { variants: PortDefinition }},
              destination: { is: 'EnumInput', label: 'Destination', props: { variants: PortDefinition }},
            },
          },
          'udp': {
            display: 'UDP',
            fields: {
              source: { is: 'EnumInput', label: 'Source', props: { variants: PortDefinition }},
              destination: { is: 'EnumInput', label: 'Destination', props: { variants: PortDefinition }},
            },
          },
          'icmp': {
            display: 'ICMP',
            fields: {
              icmp_code: { is: 'NumberBox', label: 'ICMP Code'},
            },
          },
          'group': {
            display: 'Group',
            fields: {
              members: { is: 'MultiSelect', label: 'Members', props: { searchProvider: GetServices}},
            },
          },
        }}},
        comment: { is: 'TextBox', label: 'Comment'},
      },
    },
  },
  'service': {
    name: 'Service',
    'dhcp_servers': {
      name: 'DHCP Server',
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
    'ntp_servers': {
      name: 'NTP Server',
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
    'dns_servers': {
      name: 'DNS Server',
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
    name: 'VPN',
    'wireguard_interfaces': {
      name: 'Wireguard Interface',
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
    'wireguard_peers': {
      name: 'Wireguard Peer',
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
    name: 'System',
    'users': {
      name: 'User',
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