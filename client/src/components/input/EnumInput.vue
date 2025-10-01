<script setup lang="ts">
import type { MaybeEnumValue, Variants } from './input';
import { equals, variantOf, Index } from '~/util';

const props = withDefaults(defineProps<{
  // Two-Way Bindings
  modelValue?: MaybeEnumValue,
  default?: MaybeEnumValue,

  // One-Way Bindings
  variants?: Variants,
  label?: string,
}>(), {
  default: null,
  variants: () => ({}),
  label: '',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: MaybeEnumValue): void
}>();

function getDefault() {
  if (!props.default || typeof props.default !== 'object') return props.default;
  else return Object.assign({}, props.default);
}

// Local Variables for Two-Way bindings
let mVal = $ref(props.modelValue !== undefined ? props.modelValue : getDefault());
const firstVariant = Object.keys(props.variants)[0];
const currentVariant = $computed(() => variantOf(mVal));
const savedVariants = $ref({} as Record<Index, Record<Index, any>>);
if (!mVal && firstVariant) changeVariant(firstVariant);
// Sync from v-model
onMounted(() => {
  watch(() => props.modelValue, (val) => {
    if (equals(val, mVal)) return;
    const oldVariant = variantOf(mVal);
    if (mVal && typeof mVal === 'object' && oldVariant) savedVariants[oldVariant] = mVal[oldVariant];
    mVal = val !== undefined ? val : getDefault();
  }, { deep: true, immediate: true });
});
// Sync to v-model
watch($$(mVal), () => {
  if (equals(mVal, props.modelValue)) return;
  if (mVal !== undefined) emit('update:modelValue', mVal);
}, { deep: true, immediate: true });

function changeVariant(variant: Index) {
  const oldVariant = variantOf(mVal);
  if (mVal && typeof mVal === 'object' && oldVariant) savedVariants[oldVariant] = mVal[oldVariant];

  if (!variant || !props.variants[variant].fields) return void(mVal = variant);
  mVal = { [variant]: savedVariants[variant] ?? Object.assign({}, props.variants[variant].default) ?? null };
}
</script>

<template>
  <div class="form">
    <label v-text="label"/>
    <div class="pillbar">
      <button v-for="[index, variant] of Object.entries(props.variants)" :key="index" :class="{selected: currentVariant === index}" @click="() => changeVariant(index)">
        <component :is="variant.icon" v-if="variant.icon"/>
        <template v-else>{{ variant.display }}</template>
      </button>
    </div>
    <NicerForm v-if="mVal && typeof mVal === 'object' && currentVariant" :key="currentVariant" v-model="mVal[currentVariant]" :fields="props.variants[currentVariant].fields"/>
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
