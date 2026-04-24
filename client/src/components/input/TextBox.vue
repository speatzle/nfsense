<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue?: string;
    default?: string;
  }>(),
  {
    default: "",
  },
);
const emit = defineEmits<{ (e: "update:modelValue", value: string): void }>();

let modelValue = $ref(props.modelValue ?? props.default);
syncModel(toRef(props, "modelValue"), $$(modelValue), (v) => emit("update:modelValue", v));
</script>

<template>
  <input v-model="modelValue" />
</template>

<style scoped></style>
