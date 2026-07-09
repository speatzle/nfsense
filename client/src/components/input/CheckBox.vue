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

let $mVal = props.modelValue ?? props.default;
syncModel(toRef(props, "modelValue"), $$($mVal), (v) => emit("update:modelValue", v));
</script>

<template>
  <div tabindex="0" @click="() => ($mVal = !$mVal)" @keypress="() => ($mVal = !$mVal)">
    <i-material-symbols-check-box class="on" v-if="$mVal" />
    <i-material-symbols-check-box-outline-blank v-else />
  </div>
</template>

<style scoped>
div {
  cursor: pointer;
  --cl-z: 2;
}
svg {
  & > * {
    color: var(--cl-bd);
  }

  &.on {
    --cl-fg-l: var(--cl-accent-fg-l);
    --cl-base: var(--cl-primary);
    background: radial-gradient(
      circle,
      var(--cl-fg) 0%,
      var(--cl-fg) 50%,
      #00000000 50%,
      #00000000 100%
    );

    & > * {
      color: var(--cl-bg);
      --cl-bg-fin-l: var(--cl-accent-bg-l);
      --cl-base: var(--cl-primary);
    }
  }
}
</style>
