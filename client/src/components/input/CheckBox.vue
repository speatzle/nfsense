<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue?: boolean;
    default?: boolean;
  }>(),
  {
    default: false,
  },
);
const emit = defineEmits<{ (e: "update:modelValue", value: boolean): void }>();

let mVal = $ref(props.modelValue ?? props.default);
syncModel(toRef(props, "modelValue"), $$(mVal), (v) => emit("update:modelValue", v));
</script>

<template>
  <div tabindex="0" @click="() => (mVal = !mVal)" @keypress="() => (mVal = !mVal)">
    <i-material-symbols-check-box-outline v-if="mVal" />
    <i-material-symbols-check-box-outline-blank v-else />
  </div>
</template>

<style scoped>
div {
  cursor: pointer;
}
</style>
