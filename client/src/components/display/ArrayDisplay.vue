<script setup lang="ts">
const props = withDefaults(defineProps<{
  data: any[],
  component?: string,
  componentProp?: '',
  ellipsis?: number,
  empty?: string
}>(), {
  data: () => [],
  component: '',
  componentProp: '',
  ellipsis: 10,
  empty: '',
});
</script>
<template>
  <div v-if="props.data.length != 0" class="pillstack">
    <div v-for="(item, index) of props.data.slice(0, ellipsis)" :key="index" class="pill">
      <component v-bind="{[props.componentProp]: item}" :is="props.component" v-if="props.component !== ''"/>
      <template v-else>{{ item }}</template>
    </div>
    <div v-if="props.data.length >= props.ellipsis" class="pill">...</div>
  </div>
  <div v-else>
    {{ props.empty }}
  </div>
</template>
