<script setup lang="ts">

const props = defineModels<{
  modelValue?: number,
  min?: number,
  max?: number,
}>();
const { modelValue, min, max } = $(props);

</script>

<template>
  <input v-model.number="modelValue" type="number" :min="min" :max="max">
</template>

<style scoped>
</style>