<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    data: object | string;
  }>(),
  {
    data: "",
  },
);
</script>
<template>
  {{ variantOf(props.data) }}
</template>
