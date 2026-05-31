<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    data: object | string;
    definition?: { [key: string]: { path: string; component: Component | undefined } };
  }>(),
  { data: "" },
);
</script>
<template>
  {{ variantOf(props.data) }}
</template>
