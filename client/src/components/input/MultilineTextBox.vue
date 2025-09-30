<script setup lang="ts">

const props = defineModels<{
  modelValue?: string,
}>();
const { modelValue } = $(props);

</script>

<template>
  <textarea v-model="modelValue" rows="5"/>
</template>

<style scoped>
textarea {
  resize: vertical;
}
</style>