<script setup lang="ts">

const props = defineModel<{
  options: {
    name: string,
    key: string,
    icon: Component,
  }[],
  modelValue: number | string,
  useIndex: boolean,
}>();
let { options, modelValue, useIndex } = $(props);

function setSelection(option: any, index: number){
  if (useIndex) {
      modelValue = index
    } else {
      modelValue = option.key
    }
}

onMounted(async() => {
  if (modelValue === undefined) {
    if (useIndex) {
      modelValue = 0
    } else {
      modelValue = options[0].key
    }
  }
});

</script>

<template>
  <div>
    <button class="option" v-for="(option, index) in options" :key="index" :class="{selected:  modelValue == index || modelValue == option.key}" @click="setSelection(option, index)">
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