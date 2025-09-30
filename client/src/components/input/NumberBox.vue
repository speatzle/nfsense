<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: number,
  min?: number,
  max?: number,
}>(), {
  modelValue: 0,
});
const emit = defineEmits<{(e: 'update:modelValue', value: number): void}>();

let modelValue = $ref(0);
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val; });
watch($$(modelValue), (val) => emit('update:modelValue', val));
</script>

<template>
  <input v-model.number="modelValue" type="number" :min="min" :max="max">
</template>

<style scoped>
</style>
