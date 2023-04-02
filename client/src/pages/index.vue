<script setup lang="ts">
import { apiCall } from "../api";
import PillBar from "../components/inputs/PillBar.vue";
import TextBox from "../components/inputs/TextBox.vue";
import MultilineTextBox from "../components/inputs/MultilineTextBox.vue";
import CheckBox from "../components/inputs/CheckBox.vue";
import { Form as ValidationForm, Field, ErrorMessage  } from 'vee-validate';

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
    <ValidationForm class="form">
      <Field name="email" as="TextBox" />
      <ErrorMessage name="email" />
      <Field name="email" as="MultilineTextBox" />
      <ErrorMessage name="email" />
    </ValidationForm>
  </div>
</template>

<style scoped>
.form {
  display: grid;
  grid-template-columns: auto 1fr;
  padding: 0.5rem;
  gap: 0.5rem;
}
.form > :is(button, .button, h2) {
  grid-column: 1 / 3;
}
.form > :is(label) {
  grid-column: 1;
}
</style>