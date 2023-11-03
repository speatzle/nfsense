<script setup lang="ts">
import { NavRoute } from './NavElements.vue';

withDefaults(defineProps<{
  icon?: string | Component,
  caption?: string,
  children: NavRoute[],
  clickHandler?: () => void,
}>(), {
  icon: '',
  caption: '',
  children: () => [],
  clickHandler: () => {},
});

let expanded = $ref(false);

function tallyChildren(routes: NavRoute[]) {
  let count = routes.length;
  for (const route of routes)
    if (route.children)
      count += tallyChildren(route.children);
  return count;
}

</script>
<template>
  <div :class="{'nav-dropdown-expanded': expanded}" :style="`--predicted-height: ${2.5 * tallyChildren(children)}rem;`">
    <div class="button" @click="expanded = !expanded">
      <component v-if="(typeof icon !== 'string')" :is="icon"/>
      <template v-else>{{ icon }}</template>
      <span v-text="caption"/>
      <i-material-symbols-expand-more class="nav-dropdown-expand-icon" width="1em" height="1em"/>
    </div>
    <div class="nav-dropdown-body">
      <NavElements :routes="children" :click-handler="clickHandler"/>
    </div>
  </div>
</template>
<style>
span {
  flex-grow: 1;
}
.nav-dropdown-body > :is(button, .button) {
  padding-left: calc(0.5rem - 1px);
}

.nav-dropdown-body {
  transition: all 0.1s ease-out;
  max-height: 0px;
  border-left: 1px solid white;
}
.nav-dropdown-expanded > .nav-dropdown-body {
  max-height: var(--predicted-height);
}

.nav-dropdown-expand-icon {
  transition: all 0.1s ease-out;
}
.nav-dropdown-expanded > div > .nav-dropdown-expand-icon {
  transform: rotate(180deg);
}
</style>