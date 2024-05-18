<script lang="ts">
export type NavRoute = {
  icon?: Component,
  caption?: string,
  children?: NavRoute[],
  href?: string,
};
</script>
<script setup lang="ts">
withDefaults(defineProps<{
  routes?: NavRoute[],
  clickHandler?: () => void,
}>(), {
  routes: () => [],
  clickHandler: () => {},
});

const emit = defineEmits<{
  (e: 'update:expandedDepth', value: number): void,
}>();
const lowerDepths: {[index: number]: number} = $ref({});
watch($$(lowerDepths), () => emit('update:expandedDepth', Math.max(...Object.entries(lowerDepths).map(x => x[1])) ?? 0),  { deep: true });

</script>
<template>
  <template v-if="routes">
    <template v-for="[index, route] of routes.entries()" :key="route.href">
      <router-link v-if="route.href" :to="route.href" class="button" :title="route.caption" @click="clickHandler">
        <component :is="route.icon"/>
        {{ route.caption }}
      </router-link>
      <NavDropdown v-else v-bind="route" :click-handler="clickHandler" @update:expanded-depth="(val) => lowerDepths[index] = val"/>
    </template>
  </template>
</template>