<script setup lang="ts">
import { type NavRoute } from "./navRoutes";

const props = defineProps<{
  routes: NavRoute[];
  clickHandler?: () => void;
}>();

const emit = defineEmits<{ (e: "update:depth", value: number): void }>();
const $lowerDepths = [] as number[];
watch($$($lowerDepths), () => emit("update:depth", Math.max(...$lowerDepths) ?? 0), { deep: true });
</script>
<template>
  <div class="nav-elements">
    <NavElement
      v-for="[index, route] of props.routes.entries()"
      :key="route.href"
      :route
      @update:depth="(val) => ($lowerDepths[index] = val)"
      :click-handler="props.clickHandler ?? (() => {})"
    />
  </div>
</template>
<style scoped>
.nav-elements {
  display: grid;
  grid-template-columns: calc(var(--reduced-width) - 0.25rem) 1fr auto; /* -0.25rem adjustment is for halved 0.5rem padding */
  width: 100%;
  transition: all 0.2s ease-out;
}
</style>
