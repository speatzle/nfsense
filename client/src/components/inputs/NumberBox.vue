<script setup lang="ts">

const props = defineModel<{
  modelValue: number,
  min?: number,
  max?: number,
}>();
let { modelValue, min, max } = $(props);

</script>

<template>
  <input type="number" v-model="modelValue" :min="min" :max="max">
</template>

<style scoped>
</style>