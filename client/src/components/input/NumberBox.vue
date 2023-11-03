<script setup lang="ts">

const props = defineModels<{
  modelValue?: number,
  min?: number,
  max?: number,
}>();
let { modelValue, min, max } = $(props);

</script>

<template>
  <input type="number" v-model.number="modelValue" :min="min" :max="max">
</template>

<style scoped>
</style>