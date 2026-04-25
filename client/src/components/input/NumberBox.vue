<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue?: number;
    default?: number;
    min?: number;
    max?: number;
  }>(),
  {
    default: 0,
  },
);
const emit = defineEmits<{ (e: "update:modelValue", value: number): void }>();

let $mVal = props.modelValue ?? props.default;
syncModel(toRef(props, "modelValue"), $$($mVal), (v) => emit("update:modelValue", v));
</script>

<template>
  <input v-model.number="$mVal" type="number" :min="min" :max="max" />
</template>

<style scoped></style>
