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
</script>
<template>
  <template v-if="routes">
    <template v-for="route of routes" :key="route.href">
      <router-link v-if="route.href" :to="route.href" class="button" @click="clickHandler" :title="route.caption">
        <component :is="route.icon"/>
        {{ route.caption }}
      </router-link>
      <NavDropdown v-else v-bind="route"/>
    </template>
  </template>
</template>