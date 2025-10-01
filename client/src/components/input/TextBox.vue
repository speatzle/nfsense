<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue?: string,
  default?: string,
}>(), {
  default: '',
});
const emit = defineEmits<{(e: 'update:modelValue', value: string): void}>();

let modelValue = $ref(props.modelValue ?? props.default);
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val ?? props.default; });
watch($$(modelValue), (val) => emit('update:modelValue', val), { immediate: true });

</script>

<template>
  <input v-model="modelValue">
</template>

<style scoped>
</style>
