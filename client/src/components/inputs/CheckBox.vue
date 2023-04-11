<script setup lang="ts">

const props = defineModel<{
  modelValue: boolean,
}>();
let { modelValue } = $(props);

</script>

<template>
  <div @click="() => modelValue = !modelValue">
    <i-material-symbols-check-box-outline v-if="modelValue"/>
    <i-material-symbols-check-box-outline-blank v-else/>
  </div>
</template>

<style scoped>
div {
  cursor: pointer;
}
</style>