<script setup lang="ts">
import PillBar from "../../../../components/inputs/PillBar.vue";
import TextBox from "../../../../components/inputs/TextBox.vue";
import MultilineTextBox from "../../../../components/inputs/MultilineTextBox.vue";
import CheckBox from "../../../../components/inputs/CheckBox.vue";

const props = $defineProps<{subsystem: string, entity: string, id: string}>();
const { subsystem, entity, id } = $(props);

const editTypes: { [key: string]: {[key: string]: any} } = {
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

async function update() {
  
}

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Edit ' + editTypes[subsystem][entity].title">
      <button @click="update">Update</button>
      <button @click="$router.go(-1)">Discard</button>
    </PageHeader>
    {{ subsystem }} {{ entity }} {{ id }}
    <NiceForm class="scroll cl-secondary" title="Test" :fields="editTypes[subsystem][entity].fields"/>
  </div>
  <div v-else>
    No edit for this Entity
  </div>
</template>
