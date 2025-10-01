<script setup lang="ts">
import { Index, MaybeIndex } from '../../util';

const props = withDefaults(defineProps<{
  modelValue?: MaybeIndex,
  default?: MaybeIndex,
  options: Record<Index, {
    display?: string,
    icon?: Component,
  }>,
}>(), {
  default: null,
  options: () => ({}),
});
const { options } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: MaybeIndex): void,
}>();

let modelValue = $ref(props.modelValue !== undefined ? props.modelValue : props.default);
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val !== undefined ? val : props.default; });
watch($$(modelValue), (val) => emit('update:modelValue', val), { immediate: true });

</script>

<template>
  <div>
    <button v-for="[index, option] of Object.entries(options)" :key="index" class="option" :class="{selected: modelValue === index}" @click="() => modelValue = index">
      <component :is="option.icon"/>
      {{ option.display }}
    </button>
  </div>
</template>

<style scoped>
div {
  flex-flow: nowrap;
  gap: 0.25rem;
}
button { padding: 0.25rem; gap: 0.25rem; }
.selected { background-color: var(--cl-bg-sl); }
</style>
