<script setup lang="ts">

const props = defineModel<{
  title: string
  sections: {
    title: string
    fields: {
      key: string,
      label: string,
      as: string,
      props: any,
      default: any,
      enabled?: (values: Record<string, any>) => Boolean,
      rules?: (value: any) => true | string,
    }[],
  }[],
  modelValue: any,
}>();

let { sections } = $(props);

</script>

<template>
  <ValidationForm as="div" v-slot="{ values }" @submit="false">
    <template v-for="(section, index) in sections" :key="index">
      <h4 v-if="section.title">{{ section.title }}</h4>
      <div class="section">
        <template v-for="(field, index) in section.fields" :key="index">
          <template v-if="field.enabled ? field.enabled(values) : true">
            <label :for="field.key" v-text="field.label" />
            <Field :name="field.key" :as="field.as" :rules="field.rules" v-bind="field.props" />
            <ErrorMessage :name="field.key" />
          </template>
        </template>
      </div>
    </template>
    <p>{{ values }}</p>
  </ValidationForm>
</template>

<style scoped>
.section {
  display: grid;
  grid-template-columns: auto 1fr;
  padding: 0.5rem;
  gap: 0.5rem;
}

h4,
p {
  grid-column: 1 / 3;
}

h4 {
  background-color: var(--cl-bg-hl);
  padding: 0.3rem;
  padding-left: 0.5rem;
  ;
}
</style>