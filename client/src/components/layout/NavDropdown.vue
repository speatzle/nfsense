<script setup lang="ts">
import { NavRoute } from './NavElements.vue';

const props = withDefaults(defineProps<{
  // Two-Way Bindings
  expanded?: boolean,

  // One-Way Bindings
  icon?: string | Component,
  caption?: string,
  children?: NavRoute[],
  clickHandler?: () => void,
}>(), {
  // Two-Way Bindings
  expanded: false,

  // One-Way Bindings
  icon: '',
  caption: '',
  children: () => [],
  clickHandler: () => {},
});

const emit = defineEmits<{ (e: 'update:expandedDepth', value: number): void }>();

// Local Variables for Two-Way Binding
const expanded = $ref(props.expanded ?? false);
const lowerDepth = $ref(0);

// Sync to v-model
watchEffect(() => emit('update:expandedDepth', expanded ? (lowerDepth || 0) + 1 : 0));

function tallyChildren(routes: NavRoute[]) {
  let count = routes.length;
  for (const route of routes)
    if (route.children)
      count += tallyChildren(route.children);
  return count;
}

</script>
<template>
  <div :class="{'nav-dropdown': 1, 'nav-dropdown-expanded': expanded}" :style="`--predicted-height: ${2.5 * tallyChildren(children)}rem;`">
    <div class="button" @click="expanded = !expanded">
      <component :is="icon" v-if="(typeof icon !== 'string')"/>
      <template v-else>{{ icon }}</template>
      <span>
        {{ caption }}
        <i-material-symbols-expand-more class="nav-dropdown-expand-icon" width="1em" height="1em"/>
      </span>
    </div>
    <div class="nav-dropdown-body">
      <NavElements :routes="children" :click-handler="clickHandler" @update:expanded-depth="(val) => lowerDepth = val"/>
    </div>
  </div>
</template>
<style scoped>
.nav-dropdown-body {
  max-height: 0px;
  border-left: 2px solid var(--cl-fg);
  backdrop-filter: brightness(75%);
  padding-left: calc(0.5rem - 2px);
}

span {
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  justify-content: space-between;
}

.nav-dropdown-expand-icon { min-width: 1.5rem; min-height: 1.5rem; }

/* Expanded State with Transitions */
.nav-dropdown-expand-icon, .nav-dropdown-body { transition: all 0.1s ease-out; }
.nav-dropdown-expanded > div > span > .nav-dropdown-expand-icon { transform: rotate(180deg); }
.nav-dropdown-expanded > .nav-dropdown-body { max-height: var(--predicted-height); }
</style>
