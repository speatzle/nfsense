<script setup lang="ts">
import { apiCall } from "../api";
import { Form as ValidationForm, Field, ErrorMessage  } from 'vee-validate';

async function doShit(){
  apiCall("Firewall.GetForwardRules", {});
}

let fields = $ref([
  {key: "name", label: "Name", as: "TextBox", rules: validateEmail},
  {key: "verdict", label: "Verdict", as: "PillBar", props: {options: [{name: 'Accept'}, {name: 'Drop'}, {name: 'Continue'}]}},
  {key: "counter", label: "Counter", as: "CheckBox" },
  {key: "comment", label: "Comment", as: "MultilineTextBox", enabled: (values: Record<string, any>) => (values["verdict"] == 2) as Boolean },
]);


function validateEmail(value: any) {
  // if the field is empty
  if (!value) {
    return 'This field is required';
  }

  // if the field is not a valid email
  const regex = /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}$/i;
  if (!regex.test(value)) {
    return 'This field must be a valid email';
  }

  // All is good
  return true;
}



</script>

<template>
  <div>
    <PageHeader title="Dashboard">
      <button @click="doShit">Example Buttons</button>
    </PageHeader>
    <ValidationForm class="form" v-slot="{ values }">
      <template v-for="(field, index) in fields" :key="index">
        <template v-if="field.enabled ? field.enabled(values) : true">
          <label :for="field.key" v-text="field.label"/>
          <Field :name="field.key" :as="field.as" :rules="field.rules" v-bind="field.props"/>
          <ErrorMessage :name="field.key" />
        </template>
      </template>
      {{ values }}
    </ValidationForm>
  </div>
</template>

<style scoped>
ValidationForm {
  display: grid;
  grid-template-columns: auto 1fr;
  padding: 0.5rem;
  gap: 0.5rem;
}
ValidationForm > :is(button, .button, h2) {
  grid-column: 1 / 3;
}
ValidationForm > :is(label) {
  grid-column: 1;
}
</style>