<script setup lang="ts">
import { apiCall } from "../api";
import PillBar from "../components/inputs/PillBar.vue";
import TextBox from "../components/inputs/TextBox.vue";
import MultilineTextBox from "../components/inputs/MultilineTextBox.vue";
import CheckBox from "../components/inputs/CheckBox.vue";

async function doShit(){
  apiCall("Firewall.GetForwardRules", {});
}

let fields = $ref([
  {key: "name", label: "Name", component: () => TextBox },
  {key: "verdict", label: "Verdict", component: () => PillBar, props: {options: [{name: 'Accept'}, {name: 'Drop'}, {name: 'Continue'}]}},
  {key: "counter", label: "Counter", component: () => CheckBox },
  {key: "comment", label: "Comment", component: () => MultilineTextBox },
]);

</script>

<template>
  <div>
    <PageHeader title="Dashboard">
      <button @click="doShit">Example Buttons</button>
    </PageHeader>
    <NiceForm class="scroll cl-secondary" title="Testasdfdsfsdf" :fields="fields"/>
  </div>
</template>

<style scoped>

</style>