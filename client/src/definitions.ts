export const editTypes: { [key: string]: { [key: string]: any } } = {
  "firewall": {
    "forwardrules": {
      title: "Forward Rule",
      sections: [
        { 
          fields: [
          { key: "name", label: "Name", as: "TextBox" },
          { key: "verdict", label: "Verdict", as: "PillBar", props: { options: [{ name: 'Accept' }, { name: 'Drop' }, { name: 'Continue' }] } },
          { key: "counter", label: "Counter", as: "CheckBox", },
          { key: "comment", label: "Comment", as: "MultilineTextBox", },
        ],
      },
      ],
    },
  },
  "network": {
    "interfaces": {
      title: "Interfaces",
      sections: [
        {
          fields: [
            { key: "name", label: "Name", as: "TextBox", default: "placeholder" },
            { key: "type", label: "Type", as: "PillBar", props: { options: [{ name: 'Hardware' }, { name: 'VLAN' }, { name: 'Bond' }, { name: 'Bridge' }] } },
            { key: "hardware_interface", label: "Hardware Interface", as: "TextBox", enabled: (values: any) => (values["type"] == 0) },
            { key: "vlan_id", label: "VLAN ID", as: "NumberBox", props: { min: 1, max: 4094 }, enabled: (values: any) => (values["type"] == 1) },
            { key: "bond_members", label: "Bond Members", as: "TextBox", enabled: (values: any) => (values["type"] == 2) },
            { key: "bridge_members", label: "Bridge Members", as: "TextBox", enabled: (values: any) => (values["type"] == 3) },
          ],
        },
        {
          title: "Addressing",
          fields: [
            { key: "addressing_mode", label: "Addressing Mode", as: "PillBar", props: { options: [{ name: 'None', selected: true }, { name: 'Static' }, { name: 'DHCP' }] } },
            { key: "address", label: "Address", as: "TextBox", enabled: (values: any) => (values["addressing_mode"] == 1) },
            { key: "comment", label: "Comment", as: "MultilineTextBox" },
          ],
        }
      ],
    },
  },
};