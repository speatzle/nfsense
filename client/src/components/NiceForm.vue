<script setup lang="ts">
import { loadConfigFromFile } from 'vite';


const props = defineModel<{
  title: string
  fields: {
    key: string,
    label: string,
    component: () => Component,
    props: any,
    default: any,
    if?: () => boolean,
  }[]
  modelValue: any,
}>();

let { title, fields, modelValue } = $(props);

let enabled = $ref({name: true} as any);

const emit = defineEmits<{
  (event: 'update:modelValue'): void,
  (event: 'update:enabled'): void
}>();

onMounted(async() => {
  for (const field of fields) {
    if (modelValue[field.key] == undefined) {
      if (!field.default) {
        modelValue[field.key] = {};
      } else {
        modelValue[field.key] = field.default;
      }
      enabled[field.key] = true;
    }
  }
  
  console.log("modelValue", modelValue, enabled);
});
</script>

<template>
  <div class="form">
    <h2>{{ title }}</h2>
    <template v-for="(field, index) in fields" :key="index">
      <template v-if="enabled[field.key]">
        <label :for="field.key" v-text="field.label"/>
        <component :name="field.key" :is="field.component()" v-bind="field.props" v-model="modelValue[field.key]"/>
      </template>
    </template>
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