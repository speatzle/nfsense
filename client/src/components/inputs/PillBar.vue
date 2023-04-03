<script setup lang="ts">

const props = defineModel<{
  options: {
    name: string,
    icon: Component,
    selected: boolean,
  }[],
  modelValue: number,
}>();
let { options, modelValue } = $(props);

onMounted(async() => {
  if (modelValue === undefined) {
    modelValue = 0
  }
});

</script>

<template>
  <div>
    <button class="option" v-for="(option, index) in options" :key="index" :class="{selected: modelValue == index}" @click="modelValue = index">
      <i class="material-icons" v-if="option.icon">{{ option.icon }}</i>
      {{ option.name }}
    </button>
  </div>
</template>

<style scoped>
div {
  flex-flow: nowrap;
}

.selected {
  background-color: var(--cl-bg-sl);
}
</style>