<script setup lang="ts">
const props = withDefaults(defineProps<{
  data: any[],
  component?: string,
  componentProp?: '',
  ellipsis?: number
}>(), {
  data: () => [],
  component: '',
  componentProp: '',
  ellipsis: 2,
});


</script>
<template>
  <div class="pillbar">
    <div v-for="(item, index) of props.data.slice(0, ellipsis)" :key="index" class="pill">
      <component v-bind="{[props.componentProp]: item}" :is="props.component" v-if="props.component !== ''"/>
      <template v-else>{{ item }}</template>
    </div>
    <div v-if="props.data.length >= props.ellipsis" class="pill">...</div>
  </div>
</template>
<style scoped>
  .pillbar {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
  .pill {
    border: 1px solid var(--cl-fg);
    padding: 0.25rem;
  }
</style>