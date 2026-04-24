<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue?: MaybeIndex;
    default?: MaybeIndex;
    options: Record<
      Index,
      {
        display?: string;
        icon?: Component;
      }
    >;
  }>(),
  {
    default: null,
    options: () => ({}),
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: MaybeIndex): void;
}>();

let modelValue = $ref(props.modelValue !== undefined ? props.modelValue : props.default);
syncModel(toRef(props, "modelValue"), $$(modelValue), (v) => emit("update:modelValue", v));
</script>

<template>
  <div>
    <button
      v-for="[index, option] of Object.entries(props.options)"
      :key="index"
      class="option"
      :class="{ selected: modelValue === index }"
      @click="() => (modelValue = index)"
    >
      <component :is="option.icon" />
      {{ option.display }}
    </button>
  </div>
</template>

<style scoped>
div {
  flex-flow: nowrap;
  gap: 0.25rem;
}
button {
  padding: 0.25rem;
  gap: 0.25rem;
}
.selected {
  background-color: var(--cl-bg-sl);
}
</style>
