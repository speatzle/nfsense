<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: string,
}>(), {
  modelValue: '',
});
const emit = defineEmits<{(e: 'update:modelValue', value: string): void}>();

let modelValue = $ref('');
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val; });
watch($$(modelValue), (val) => emit('update:modelValue', val));
</script>

<template>
  <textarea v-model="modelValue" rows="5"/>
</template>

<style scoped>
textarea {
  resize: vertical;
}
</style>
