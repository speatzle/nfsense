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

const emit = defineEmits<{
  (event: 'selectionChanged'): void
}>();

function select(option: any, index: number) {
  for(let opt of options) {
    opt.selected = false;
  }
  option.selected = true;
  modelValue = index;
  emit('selectionChanged');
  console.debug("selected", options);
}

</script>

<template>
  <div>
    <button class="option" v-for="(option, index) in options" :key="index" :class="{selected:option.selected}" @click="select(option, index)">
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