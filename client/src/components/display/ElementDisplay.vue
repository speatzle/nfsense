<script setup lang="ts">
const props = withDefaults(defineProps<{
  data: any,
  component?: string,
  componentProp?: '',
}>(), {
  data: '',
  component: '',
  componentProp: '',
});
</script>
<template>
  <div v-if="data" class="pillbar">
    <div class="pill">
      <component v-bind="{[props.componentProp]: props.data}" :is="props.component" v-if="props.component !== ''"/>
      <template v-else>{{ props.data }}</template>
    </div>
  </div>
</template>
