<script setup lang="ts">
import { Index, MaybeIndex } from '../../util';

const props = withDefaults(defineProps<{
  options: Record<Index, {
    display?: string,
    icon?: Component,
  }>,
  modelValue: MaybeIndex,
}>(), {
  options: () => ({}),
});
const { options } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: MaybeIndex): void,
}>();

let modelValue: MaybeIndex = $ref(null);
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val; });
watch($$(modelValue), (val) => emit('update:modelValue', val));

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