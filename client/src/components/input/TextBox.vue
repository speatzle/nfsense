<script setup lang="ts">

const props = defineModels<{
  modelValue?: string,
}>();
const { modelValue } = $(props);

</script>

<template>
  <input v-model="modelValue">
</template>

<style scoped>
</style>