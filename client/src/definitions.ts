import { toFormValidator } from '@vee-validate/zod';
import * as zod from 'zod';
import { SearchProvider, Options } from '~/components/input/DropdownInput.vue';
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
  let res = await apiCall('object.services.list', {});
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
  let res = await apiCall('vpn.wireguard.peers.list', {});
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
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        source_addresses: { is: 'MultiSelect', label: 'Source', props: { searchProvider: GetAddresses}},
        destination_addresses: { is: 'MultiSelect', label: 'Destination', props: { searchProvider: GetAddresses}},
        services: { is: 'MultiSelect', label: 'Services', props: { searchProvider: GetServices}},
        verdict: { is: 'EnumInput', label: 'Verdict', props: { variants: {
          'accept': { display: 'Accept'},
          'drop': { display: 'Drop'},
          'continue': { display: 'Continue'},
        }}},
        counter: { is: 'CheckBox', label: 'Counter'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'destination_nat_rules': {
      name: 'Destination NAT Rule',
      idType: 'Number',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        source_addresses: { is: 'MultiSelect', label: 'Source', props: { searchProvider: GetAddresses}},
        destination_addresses: { is: 'MultiSelect', label: 'Destination', props: { searchProvider: GetAddresses}},
        services: { is: 'MultiSelect', label: 'Services', props: { searchProvider: GetServices}},
        dnat_heading: { is: 'Heading', props: { caption: 'DNAT' }},
        dnat_address: { is: 'SingleSelect', label: 'Destination', props: { searchProvider: GetAddresses}},
        dnat_service: { is: 'SingleSelect', label: 'Service', props: { searchProvider: GetServices}},
        counter: { is: 'CheckBox', label: 'Counter'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'source_nat_rules': {
      name: 'Source NAT Rule',
      idType: 'Number',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        source_addresses: { is: 'MultiSelect', label: 'Source', props: { searchProvider: GetAddresses}},
        destination_addresses: { is: 'MultiSelect', label: 'Destination', props: { searchProvider: GetAddresses}},
        services: { is: 'MultiSelect', label: 'Services', props: { searchProvider: GetServices}},
        snat_heading: { is: 'Heading', props: { caption: 'SNAT' }},
        snat_type: { is: 'EnumInput', label: 'Type', props: { variants: {
          'masquerade': { display: 'Masquerade' },
          'snat': {
            display: 'SNAT',
            fields: {
              address: { is: 'SingleSelect', label: 'Destination', props: { searchProvider: GetAddresses}},
              service: { is: 'SingleSelect', label: 'Service', props: { searchProvider: GetServices}},
            },
          },
        }}},
        counter: { is: 'CheckBox', label: 'Counter'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
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
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'static_routes': {
      name: 'Static Route',
      idType: 'Number',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        gatway: { is: 'TextBox', label: 'Gateway'},
        destination: { is: 'TextBox', label: 'Destination'},
        metric: { is: 'NumberBox', label: 'Metric'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
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
        comment: { is: 'MultilineTextBox', label: 'Comment'},
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
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
  },
  'service': {
    name: 'Service',
    'dhcp_servers': {
      name: 'DHCP Server',
      idType: 'Number',
      fields: {
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        pool: { is: 'MultiSelect', label: 'Pool', props: { searchProvider: GetAddresses} },
        gateway_mode: { is: 'EnumInput', label: 'Gateway Mode', props: { variants: {
          'none': { display: 'None' },
          'interface': { display: 'Interface' },
          'specify': {
            display: 'Specify',
            fields: {
              gateway: { is: 'SingleSelect', label: 'Gateway', props: { searchProvider: GetAddresses} },
            },
          },
        }}},
        dns_server_mode: { is: 'EnumInput', label: 'DNS Server Mode', props: { variants: {
          'none': { display: 'None' },
          'interface': { display: 'Interface' },
          'specify': {
            display: 'Specify',
            fields: {
              servers: { is: 'MultiSelect', label: 'DNS Servers', props: { searchProvider: GetAddresses} },
            },
          },
        }}},
        ntp_server_mode: { is: 'EnumInput', label: 'NTP Server Mode', props: { variants: {
          'none': { display: 'None' },
          'interface': { display: 'Interface' },
          'specify': {
            display: 'Specify',
            fields: {
              servers: { is: 'MultiSelect', label: 'NTP Servers', props: { searchProvider: GetAddresses} },
            },
          },
        }}},
        default_lease_time: { is: 'NumberBox', label: 'Default Lease Time'},
        max_lease_time: { is: 'NumberBox', label: 'Max Lease Time'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'ntp_servers': {
      name: 'NTP Server',
      idType: 'Number',
      fields: {
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'dns_servers': {
      name: 'DNS Server',
      idType: 'Number',
      validationSchema: toFormValidator(
        zod.object({
        }),
      ),
      fields: {
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
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