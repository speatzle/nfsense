<script setup lang="ts">
import type { MaybeEnumValue, Variants, EnumValueWithFields } from './input';
import { equals, variantOf, MaybeIndex, Index } from '~/util';

const props = withDefaults(defineProps<{
  // Two-Way Bindings
  modelValue?: MaybeEnumValue,

  // One-Way Bindings
  variants?: Variants,
  label?: string,
}>(), {
  modelValue: null,
  variants: () => ({}),
  label: '',
});
const { variants } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: MaybeEnumValue): void
}>();

// Local Variables for Two-Way bindings
let modelValue = $ref(null as MaybeEnumValue);
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
watch($$(modelValue), (val, oldVal) => {
  if (equals(val, props.modelValue)) return;
  const [oldVariant, newVariant] = [variantOf(oldVal), variantOf(val)];
  // Wipe variant values if variant definitions change
  if (typeof val === 'object'
    && val && newVariant && oldVariant
    && !equals(props.variants[newVariant].fields, props.variants[oldVariant].fields))
    val[newVariant] = formValue = {};
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
  <div class="form">
    <label v-text="label"/>
    <div class="pillbar">
      <button v-for="[index, variant] of Object.entries(variants)" :key="index" :class="{selected: currentVariant === index}" @click="() => currentVariant = index">
        <component :is="variant.icon" v-if="variant.icon"/>
        <template v-else>{{ variant.display }}</template>
      </button>
    </div>
    <NicerForm v-if="currentVariant && variants[currentVariant]?.fields" :key="currentVariant" v-model="formValue" :fields="variants[currentVariant].fields"/>
  </div>
</template>
<style scoped>
.pillbar {
  flex-flow: nowrap;
  gap: 0.25rem;
}
.pillbar > button { padding: 0.25rem; gap: 0.25rem; }
.pillbar > .selected {
  background-color: var(--cl-bg-sl);
  border: 1px solid var(--cl-fg);
  padding: calc(0.25rem - 1px);
}
</style>
