<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    headingLevel?: number;
    caption?: string;
  }>(),
  {
    headingLevel: 2,
    caption: "",
  },
);
</script>
<template>
  <component :is="`h` + props.headingLevel">{{ props.caption }}</component>
</template>
