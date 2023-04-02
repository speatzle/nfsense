<script setup lang="ts">
import PillBar from "../../../../components/inputs/PillBar.vue";
import TextBox from "../../../../components/inputs/TextBox.vue";
import MultilineTextBox from "../../../../components/inputs/MultilineTextBox.vue";
import CheckBox from "../../../../components/inputs/CheckBox.vue";

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

const createTypes: { [key: string]: {[key: string]: any} } = {
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

async function create() {
  
}

</script>
<template>
  <div v-if="createTypes[subsystem][entity]">
    <PageHeader :title="'Edit ' + createTypes[subsystem][entity].title">
      <button @click="create">Create</button>
      <button @click="$router.go(-1)">Discard</button>
    </PageHeader>
    {{ subsystem }} {{ entity }}
    <NiceForm class="scroll cl-secondary" title="Test" :fields="createTypes[subsystem][entity].fields"/>
  </div>
  <div v-else>
    No create for this Entity
  </div>
</template>
