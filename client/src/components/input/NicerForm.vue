<script lang="ts">
import { equals, Index } from '../../util';

export type Field = {
  is: Component | string,
  label?: string,
  props?: any,
  // actions?: Action[],
  // forceCastNumber`?: bool,
};
export type Fields = Record<Index, Field>;
</script>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  // Two-Way Bindings
  modelValue?: Record<Index, any>,

  // One-Way Bindings
  fields?: Fields,
  heading?: string,
  headingLevel?: number,
}>(), {
  modelValue: () => ({}),
  fields: () => ({}),
  heading: '',
  headingLevel: 2,
});
const { fields } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: Record<Index, any>): void
}>();

// Local Variables for Two-Way bindings
let modelValue: Record<Index, any> = $ref({});
// Sync from v-model
onMounted(() => {
  watch(() => props.modelValue, (val) => {
    if (equals(val, modelValue)) return;
    modelValue = Object.assign(modelValue, val);
  }, { deep: true, immediate: true });
});
// Sync to v-model
watch($$(modelValue), (val) => {
  if (equals(val, props.modelValue)) return;
  emit('update:modelValue', Object.assign({}, val));
}, { deep: true });
</script>

<template>
  <div class="form">
    <component v-if="heading" :is="`h${headingLevel}`">{{ heading }}</component>
    <div class="form inner-form">
      <template v-for="[index, field] of Object.entries(fields)" :key="index">
        <label v-if="field.label && field.is !== 'EnumInput'" v-text="field.label"/>
        <component :is="field.is" v-model="modelValue[index]" v-bind="field.is === 'EnumInput' ? Object.assign({label: field.label}, field.props) : field.props"/>
      </template>
    </div>
  </div>
</template>
