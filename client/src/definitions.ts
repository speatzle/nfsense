import PillBar from "./components/inputs/PillBar.vue";
import TextBox from "./components/inputs/TextBox.vue";
import NumberBox from "./components/inputs/NumberBox.vue";
import MultilineTextBox from "./components/inputs/MultilineTextBox.vue";
import CheckBox from "./components/inputs/CheckBox.vue";

export const editTypes: { [key: string]: {[key: string]: any} } = {
    "firewall": {
      "forwardrules": {
        title: "Forward Rule",
        fields: [
          {key: "name", label: "Name", component: () => TextBox },
          {key: "verdict", label: "Verdict", component: () => PillBar, props: {options: [{name: 'Accept'}, {name: 'Drop'}, {name: 'Continue'}]}},
          {key: "counter", label: "Counter", component: () => CheckBox },
          {key: "comment", label: "Comment", component: () => MultilineTextBox },
        ],
      }
    },
    "network": {
      "interfaces": {
        title: "Interfaces",
        fields: [
          {key: "name", label: "Name", component: () => TextBox, default: "placeholder"},
          {key: "type", label: "Type", component: () => PillBar, props: {options: [{name: 'Hardware', selected: true}, {name: 'VLAN'}, {name: 'Bond'}, {name: 'Brdige'}]}},
          {key: "hardware_interface", label: "Hardware Interface", component: () => TextBox },
          {key: "vlan_id", label: "VLAN ID", component: () => NumberBox, props: {min: 1, max: 4094} },
          {key: "bond_members", label: "Bond Members", component: () => TextBox, if: () => true },
          {key: "bridge_members", label: "Bridge Memebers", component: () => TextBox },
          {key: "addressing_mode", label: "Addressing Mode", component: () => PillBar, props: {options: [{name: 'None', selected: true}, {name: 'Static'}, {name: 'DHCP'}]}},
          {key: "address", label: "Address", component: () => TextBox },
          {key: "comment", label: "Comment", component: () => MultilineTextBox },
        ],
      }
    },
  };