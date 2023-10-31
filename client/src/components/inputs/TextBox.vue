<script setup lang="ts">

const props = defineModels<{
  modelValue?: string,
}>();
let { modelValue } = $(props);

</script>

<template>
  <input v-model="modelValue">
</template>

<style scoped>
</style>