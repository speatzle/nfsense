import PillBar from "./components/inputs/PillBar.vue";
import TextBox from "./components/inputs/TextBox.vue";
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
          {key: "name", label: "Name", component: () => TextBox },
          {key: "comment", label: "Comment", component: () => MultilineTextBox },
        ],
      }
    },
  };