<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: boolean,
}>(), {
  modelValue: false,
});
const emit = defineEmits<{(e: 'update:modelValue', value: boolean): void}>();

let modelValue = $ref(false);
watch(() => props.modelValue, (val) => { if (val !== modelValue) modelValue = val; });
watch($$(modelValue), (val) => emit('update:modelValue', val));
</script>

<template>
  <div tabindex="0" @click="() => modelValue = !modelValue" @keypress="() => modelValue = !modelValue">
    <i-material-symbols-check-box-outline v-if="modelValue"/>
    <i-material-symbols-check-box-outline-blank v-else/>
  </div>
</template>

<style scoped>
div { cursor: pointer; }
</style>
