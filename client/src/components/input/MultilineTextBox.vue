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

let $mVal = props.modelValue ?? props.default;
syncModel(toRef(props, "modelValue"), $$($mVal), (v) => emit("update:modelValue", v));
</script>

<template>
  <textarea v-model="$mVal" rows="5" />
</template>

<style scoped>
textarea {
  resize: vertical;
}
</style>
