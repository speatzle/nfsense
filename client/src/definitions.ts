import { toFormValidator } from '@vee-validate/zod';
import * as zod from 'zod';

export const editTypes: { [key: string]: { [key: string]: any } } = {
  "firewall": {
    name: "Firewall",
    "forwardrules": {
      name: "ForwardRule",
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
            { key: "name", label: "Name", as: "TextBox", default: "placeholder" },
            { key: "type", label: "Type", as: "PillBar", props: { options: [{ name: 'Hardware', key: 'hardware' }, { name: 'VLAN', key: 'vlan' }, { name: 'Bond', key: 'bond' }, { name: 'Bridge', key: 'bridge' }] } },
            { key: "hardware_device", label: "Hardware Device", as: "TextBox", enabled: (values: any) => (values["type"] == 'hardware') },
            { key: "vlan_parent", label: "VLAN Parent", as: "TextBox", enabled: (values: any) => (values["type"] == 'vlan') },
            { key: "vlan_id", label: "VLAN ID", as: "NumberBox", props: { min: 1, max: 4094 }, enabled: (values: any) => (values["type"] == 'vlan') },
            { key: "bond_members", label: "Bond Members", as: "TextBox", enabled: (values: any) => (values["type"] == 'bond') },
            { key: "bridge_members", label: "Bridge Members", as: "TextBox", enabled: (values: any) => (values["type"] == 'bridge') },
          ],
        },
        {
          title: "Addressing",
          fields: [
            { key: "addressing_mode", label: "Addressing Mode", as: "PillBar", props: { options: [{ name: 'None', key: 'none' }, { name: 'Static', key: 'static' }, { name: 'DHCP', key: 'dhcp' }] } },
            { key: "address", label: "Address", as: "TextBox", enabled: (values: any) => (values["addressing_mode"] == 'static') },
            { key: "comment", label: "Comment", as: "MultilineTextBox" },
          ],
        }
      ],
    },
    "staticroutes": {
      name: "StaticRoute",
      validationSchema: toFormValidator(
        zod.object({
          name: zod.string(),
        }),
      ),
      sections: [
        {
          fields: [
            { key: "name", label: "Name", as: "TextBox", },
            { key: "interface", label: "Interface", as: "TextBox" },
            { key: "gateway", label: "Gateway", as: "TextBox" },
            { key: "destination", label: "Destination", as: "TextBox" },
            { key: "metric", label: "Metric", as: "NumberBox" },
          ],
        },
      ],
    },
  },
};