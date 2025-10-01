<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue?: number,
  default?: number,
  min?: number,
  max?: number,
}>(), {
  default: 0,
});
const emit = defineEmits<{(e: 'update:modelValue', value: number): void}>();

let mVal = $ref(props.modelValue ?? props.default);
watch(() => props.modelValue, (val) => { if (val !== mVal) mVal = val ?? props.default; });
watch($$(mVal), (val) => emit('update:modelValue', val), { immediate: true });
</script>

<template>
  <input v-model.number="mVal" type="number" :min="min" :max="max">
</template>

<style scoped>
</style>
