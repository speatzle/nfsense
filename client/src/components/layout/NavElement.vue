<script setup lang="ts">
import { type NavRoute } from "./navRoutes";

const props = defineProps<{
  route: NavRoute;
  clickHandler: () => void;
}>();

const emit = defineEmits<{ (e: "update:depth", value: number): void }>();

// Local Variables for Two-Way Binding
let $expanded = false as boolean;
const $lowerDepth = 0 as number;

// Sync to v-model
watchEffect(() => emit("update:depth", $expanded ? ($lowerDepth || 0) + 1 : 0));

const tallyChildren = (routes: NavRoute[]): number =>
  routes.reduce((tally, route) => tally + 1 + tallyChildren(route.children ?? []), 0);
</script>
<template>
  <router-link
    v-if="props.route.href"
    :to="props.route.href"
    class="button"
    :title="props.route.caption"
    @click="props.clickHandler"
  >
    <component :is="props.route.icon" />
    <span v-text="props.route.caption" />
  </router-link>
  <template v-else-if="props.route.children">
    <div class="button" @click="$expanded = !$expanded">
      <component :is="props.route.icon" />
      {{ props.route.caption }}
      <i-material-symbols-expand-more :class="{ 'expand-icon': 1, expanded: $expanded }" />
    </div>
    <NavElements
      :style="`--predicted-height: ${2.5 * tallyChildren(props.route.children)}rem;`"
      :class="{ dropdown: 1, expanded: $expanded }"
      :routes="props.route.children"
      :click-handler="props.clickHandler"
      @update:depth="(val) => ($lowerDepth = val)"
    />
  </template>
</template>
<style scoped>
:is(button, .button, .dropdown) {
  display: grid;
  grid-template-columns: subgrid;
  grid-column: 1 / -1;

  & svg {
    place-self: center;
  }
  &:not(:hover) {
    background: transparent;
  }
  & > span:last-child {
    grid-column: 2 / -1;
  }
}

.expand-icon {
  transition: transform 0.2s ease-out;
  &.expanded {
    transform: rotate(180deg);
  }
}

.dropdown {
  max-height: 0px;
  border-left: 2px solid var(--cl-fg);
  backdrop-filter: brightness(75%);
  padding-left: calc(0.5rem - 2px);

  & .button:not(:hover) {
    background: transparent;
  }
  &.expanded {
    max-height: var(--predicted-height);
  }
}
</style>
