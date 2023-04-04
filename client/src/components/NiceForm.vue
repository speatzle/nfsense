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
  submit: (value: any) => boolean,
  discard: () => void,
}>();

let { sections, submit, discard } = $(props);

</script>

<template>
  <ValidationForm as="div" v-slot="{ values, handleSubmit }" @submit="submit">
    <template v-for="(section, index) in sections" :key="index">
      <h4 v-if="section.title">{{ section.title }}</h4>
      <div class="section">
        <template v-for="(field, index) in section.fields" :key="index">
          <template v-if="field.enabled ? field.enabled(values) : true">
            <label :for="field.key" v-text="field.label" />
            <Field v-if="field.as == 'NumberBox'" :name="field.key" :as="field.as" :rules="field.rules" v-bind="field.props" @update:modelValue="values[field.key] = Number(values[field.key])"/>
            <Field v-else :name="field.key" :as="field.as" :rules="field.rules" v-bind="field.props"/>
            <ErrorMessage :name="field.key" />
          </template>
        </template>
      </div>
    </template>
    <div class="actions">
      <div class="flex-grow"/>
      <button @click="handleSubmit($event, submit)">Submit</button>
      <div class="space"/>
      <button @click="discard">Discard</button>
      <div class="flex-grow"/>
    </div>
    <p>{{ values }}</p>
  </ValidationForm>
</template>

<style scoped>
form {
  display: flex;
  flex-direction: column;
}
.section {
  display: grid;
  grid-template-columns: auto 1fr;
  padding: 0.5rem;
  gap: 0.5rem;
}
.actions {
  flex-direction: row;
}

.space {
  padding: 0.2rem;
}

.actions > button {
  flex-grow: true;
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