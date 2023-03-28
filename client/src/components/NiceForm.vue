<script setup lang="ts">

const props = defineModel<{
  title: string
  fields: {
    key: string,
    label: string,
    component: () => Component,
    props: any,
  }[]
}>();
let { title, fields } = $(props);

</script>

<template>
  <div class="form">
    <h2>{{ title }}</h2>
    <template v-for="(field, index) in fields" :key="index">
      <label :for="field.key" v-text="field.label"/>
      <component :name="field.key" :is="field.component()" v-bind="field.props"/>
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