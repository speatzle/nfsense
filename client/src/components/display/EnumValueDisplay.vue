<script setup lang="ts">
// @ts-nocheck
import { variantOf, atPath } from '~/util';
import type { Component } from 'vue';

const props = withDefaults(defineProps<{
  data: object | string,
  definition: { [key: string]: { path: string, component: Component | undefined } } | undefined,
}>(), {
  data: '',
  definition: undefined,
});

const value = computed(() => {
  const variant = variantOf(props.data);
  if (variant == null) {
    return {};
  }
  if (props.definition === undefined) {
    return {};
  }
  const thing = props.definition[variant];
  if (thing == null) {
    return {};
  }
  return { data: atPath(props.data, thing.path), component: thing.component };
});

</script>
<template>
  <component :is="value.component" v-if="value.component" :data="value.data"/>
  <div v-else>
    {{ value.data }}
  </div>
</template>
