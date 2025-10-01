<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue?: string,
  default?: string,
}>(), {
  default: '',
});
const emit = defineEmits<{(e: 'update:modelValue', value: string): void}>();

let mVal = $ref(props.modelValue ?? props.default);
watch(() => props.modelValue, (val) => { if (val !== mVal) mVal = val ?? props.default; });
watch($$(mVal), (val) => emit('update:modelValue', val), { immediate: true });
</script>

<template>
  <textarea v-model="mVal" rows="5"/>
</template>

<style scoped>
textarea {
  resize: vertical;
}
</style>
