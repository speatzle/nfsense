<script setup lang="ts">

const props = withDefaults(defineProps<{
  headingLevel?: number,
  caption?: string,
}>(), {
  headingLevel: 2,
  caption: '',
});
const { headingLevel, caption } = $(props);

</script>
<template>
  <component :is="`h` + headingLevel">{{ caption }}</component>
</template>