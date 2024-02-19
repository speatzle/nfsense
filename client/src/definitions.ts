import { SearchProvider, Options } from '~/components/input/DropdownInput.vue';
import { apiCall } from './api';

const GetHardwareInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.links.list', {});
  if (res.Error === null) {
    console.debug('links', res.Data);
    return Object.fromEntries(res.Data.map((r: any) => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetInterfaces: SearchProvider = async (o) => {
  let res = await apiCall('network.interfaces.list', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetAddresses: SearchProvider = async (o) => {
  let res = await apiCall('object.addresses.list', {});
  if (res.Error === null) {
    console.debug('addresses', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetServices: SearchProvider = async (o) => {
  let res = await apiCall('object.services.list', {});
  if (res.Error === null) {
    console.debug('services', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
  } else {
    console.debug('error', res);
    return {} as Options;
  }
};

const GetPeers: SearchProvider = async (o) => {
  let res = await apiCall('vpn.wireguard.peers.list', {});
  if (res.Error === null) {
    console.debug('peers', res.Data);
    return Object.fromEntries(res.Data.map(r => [r.name, { display: r.name }]));
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
              address: { is: 'SingleSelect', label: 'Source', props: { searchProvider: GetAddresses}},
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
              parent: { is: 'SingleSelect', label: 'VLAN Parent', props: { searchProvider: GetInterfaces}},
              id: { is: 'NumberBox', label: 'VLAN ID', props: { min: 1, max: 4094 }},
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
        gateway: { is: 'TextBox', label: 'Gateway'},
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
        name: { is: 'TextBox', label: 'Name'},
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
              dns_servers: { is: 'MultiSelect', label: 'DNS Servers', props: { searchProvider: GetAddresses} },
            },
          },
        }}},
        ntp_server_mode: { is: 'EnumInput', label: 'NTP Server Mode', props: { variants: {
          'none': { display: 'None' },
          'interface': { display: 'Interface' },
          'specify': {
            display: 'Specify',
            fields: {
              ntp_servers: { is: 'MultiSelect', label: 'NTP Servers', props: { searchProvider: GetAddresses} },
            },
          },
        }}},
        lease_time: { is: 'NumberBox', label: 'Lease Time'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'ntp_servers': {
      name: 'NTP Server',
      idType: 'Number',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'dns_servers': {
      name: 'DNS Server',
      idType: 'Number',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        interface: { is: 'SingleSelect', label: 'Interface', props: { searchProvider: GetInterfaces} },
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
  },
  'vpn': {
    name: 'VPN',
    'wireguard.interfaces': {
      name: 'Wireguard Interface',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        public_key: { is: 'TextBox', label: 'Public Key'},
        private_key: { is: 'TextBox', label: 'Private Key'},
        listen_port: { is: 'NumberBox', label: 'Listen Port'},
        peers: { is: 'MultiSelect', label: 'Peers', props: { searchProvider: GetPeers} },
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
    'wireguard.peers': {
      name: 'Wireguard Peer',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        public_key: { is: 'TextBox', label: 'Public Key'},
        preshared_key: { is: 'TextBox', label: 'Preshared Key'},
        allowed_ips: { is: 'MultiSelect', label: 'Allowed IPs', props: { searchProvider: GetAddresses} },
        endpoint: { is: 'TextBox', label: 'Endpoint'},
        persistent_keepalive: { is: 'NumberBox', label: 'Persistent Keepalive'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
  },
  'system': {
    name: 'System',
    'users': {
      name: 'User',
      fields: {
        name: { is: 'TextBox', label: 'Name'},
        password: { is: 'TextBox', label: 'Password'},
        comment: { is: 'MultilineTextBox', label: 'Comment'},
      },
    },
  },
};