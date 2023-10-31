<script lang="ts">
import { Index, MaybeIndex, equals } from '../../util';
import NicerForm, { Fields } from './NicerForm.vue';

export type Variant = {
  fields?: Fields,
  display?: string,
  icon?: Component
};
export type Variants = Record<Index, Variant>;
export type EnumValueWithFields = { [index: Index]: Record<Index, any> }
export type EnumValue = Index | EnumValueWithFields;
export type MaybeEnumValue = EnumValue | null;
</script>

<script setup lang="ts">

const props = withDefaults(defineProps<{
  // Two-Way Bindings
  modelValue?: MaybeEnumValue,

  // One-Way Bindings
  variants?: Variants,
}>(), {
  modelValue: null,
  variants: () => ({}),
});
const { variants } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: MaybeEnumValue): void
}>();

// Local Variables for Two-Way bindings
let modelValue: MaybeEnumValue = $ref(null);
// Sync from v-model
onMounted(() => {
  watch(() => props.modelValue, (val) => {
    if (equals(val, modelValue)) return;
    [currentVariant, formValue] = val ?
      typeof val === 'string'
        ? [val, {}]
        : [Object.keys(val)[0], (val as EnumValueWithFields)[Object.keys(val)[0]]]
      : [null, {}];
    if (modelValue && typeof val === 'object' && typeof modelValue === 'object') modelValue = Object.assign(modelValue, val);
    else modelValue = val;
  }, { deep: true, immediate: true });
});
// Sync to v-model
watch($$(modelValue), (val) => {
  if (equals(val, props.modelValue)) return;
  emit('update:modelValue', typeof val === 'string' ? val : Object.assign({}, val));
}, { deep: true });

let currentVariant: MaybeIndex = $ref(null);
let formValue: Record<Index, any> = $ref({});

watchEffect(() => {
  if (!currentVariant) modelValue = null;
  else modelValue = variants[currentVariant].fields
    ? { [currentVariant]: formValue ?? {} }
    : currentVariant;
});

</script>
<template>
  <div class="enum-input">
    <div class="pillbar">
      <button class="variant" v-for="[index, variant] of Object.entries(variants)" :key="index" :class="{selected: currentVariant === index}" @click="() => currentVariant = index">
        <component v-if="variant.icon" :is="variant.icon"/>
        <template v-else>{{ variant.display }}</template>
      </button>
    </div>
    <NicerForm class="enum-fields" v-if="currentVariant && variants[currentVariant]?.fields" :fields="variants[currentVariant].fields" v-model="formValue" :key="currentVariant"/>
  </div>
</template>
<style scoped>
.pillbar {
  flex-flow: nowrap;
  gap: 0.25rem;
}
.variant { padding: 0.25rem; gap: 0.25rem; }
.selected { background-color: var(--cl-bg-sl); }
</style>