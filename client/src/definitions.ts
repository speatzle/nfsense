export const editTypes: { [key: string]: { [key: string]: any } } = {
  "firewall": {
    name: "Firewall",
    "forwardrules": {
      name: "ForwardRule",
      sections: [
        { 
          fields: [
          { key: "name", label: "Name", as: "TextBox" },
          { key: "verdict", label: "Verdict", as: "PillBar", props: { options: [{ name: 'Accept', key: 'accept' }, { name: 'Drop', key: 'drop' }, { name: 'Continue', key: 'continue' }] } },
          { key: "counter", label: "Counter", as: "CheckBox", },
          { key: "comment", label: "Comment", as: "MultilineTextBox", },
        ],
      },
      ],
    },
  },
  "network": {
    name: "Network",
    "interfaces": {
      name: "Interface",
      sections: [
        {
          fields: [
            { key: "name", label: "Name", as: "TextBox", default: "placeholder" },
            { key: "type", label: "Type", as: "PillBar", props: { options: [{ name: 'Hardware', key: 'hardware' }, { name: 'VLAN', key: 'vlan' }, { name: 'Bond', key: 'bond' }, { name: 'Bridge', key: 'bridge' }] } },
            { key: "hardware_interface", label: "Hardware Interface", as: "TextBox", enabled: (values: any) => (values["type"] == 0) },
            { key: "vlan_id", label: "VLAN ID", as: "NumberBox", props: { min: 1, max: 4094 }, enabled: (values: any) => (values["type"] == 1) },
            { key: "bond_members", label: "Bond Members", as: "TextBox", enabled: (values: any) => (values["type"] == 2) },
            { key: "bridge_members", label: "Bridge Members", as: "TextBox", enabled: (values: any) => (values["type"] == 3) },
          ],
        },
        {
          title: "Addressing",
          fields: [
            { key: "addressing_mode", label: "Addressing Mode", as: "PillBar", props: { options: [{ name: 'None', key: 'none' }, { name: 'Static', key: 'static' }, { name: 'DHCP', key: 'dhcp' }] } },
            { key: "address", label: "Address", as: "TextBox", enabled: (values: any) => (values["addressing_mode"] == 1) },
            { key: "comment", label: "Comment", as: "MultilineTextBox" },
          ],
        }
      ],
    },
  },
};