import { toFormValidator } from '@vee-validate/zod';
import * as zod from 'zod';
import { SearchProvider, Options } from '~/components/inputs/DropdownInput.vue';
import { apiCall } from './api';

const GetHardwareInterfaces: SearchProvider = async (o) => {
  let res = await apiCall("Network.GetLinks", {});
  if (res.Error === null) {
    console.debug("links", res.Data.Links);
    return Object.fromEntries(res.Data.Links.map(r => [r.name, { display: r.name }]))
  } else {
    console.debug("error", res);
    return {} as Options;
  }
};

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
            { key: "verdict", label: "Verdict", as: "PillBar", props: { options: { accept: { display: 'Accept' }, drop: { display: 'Drop' }, continue: { display: 'Continue' } } } },
            { key: "counter", label: "Counter", as: "CheckBox" },
            { key: "comment", label: "Comment", as: "MultilineTextBox" },
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
            { key: "type", label: "Type", as: "PillBar", props: { options: { hardware: { display: 'Hardware' }, vlan: { display: 'VLAN' }, bond: { display: 'Bond' }, bridge: { display: 'Bridge' } } } },
            { key: "hardware_device", label: "Hardware Device", as: "SingleSelect", enabled: (values: any) => (values["type"] == 'hardware'), props: { searchProvider: GetHardwareInterfaces } },
            { key: "vlan_parent", label: "VLAN Parent", as: "SingleSelect", enabled: (values: any) => (values["type"] == 'vlan'), props: { searchProvider: GetHardwareInterfaces } },
            { key: "vlan_id", label: "VLAN ID", as: "NumberBox", props: { min: 1, max: 4094 }, enabled: (values: any) => (values["type"] == 'vlan') },
            { key: "bond_members", label: "Bond Members", as: "MultiSelect", enabled: (values: any) => (values["type"] == 'bond'), props: { searchProvider: GetHardwareInterfaces } },
            { key: "bridge_members", label: "Bridge Members", as: "MultiSelect", enabled: (values: any) => (values["type"] == 'bridge'), props: { searchProvider: GetHardwareInterfaces } },
          ],
        },
        {
          title: "Addressing",
          fields: [
            { key: "addressing_mode", label: "Addressing Mode", as: "PillBar", props: { options: { none: { display: 'None' }, static: { display: 'Static' }, dhcp: { display: 'DHCP' } } } },
            { key: "address", label: "Address", as: "TextBox", enabled: (values: any) => (values["addressing_mode"] == 'static') },
            { key: "comment", label: "Comment", as: "MultilineTextBox" },
          ],
        },
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
            { key: "name", label: "Name", as: "TextBox" },
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