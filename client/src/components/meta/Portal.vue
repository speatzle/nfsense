<script setup lang="ts">
import { $activeTargets } from "./portal";
const props = defineProps<{
  from?: string;
  to?: string;
}>();

if (props.from) {
  onMounted(() => $activeTargets.push(props.from!));
  onBeforeUnmount(() => $activeTargets.splice($activeTargets.indexOf(props.from!), 1));
}
</script>

<template>
  <div v-if="props.from" :id="'portal-' + props.from">
    <slot />
  </div>
  <Teleport v-else-if="props.to && $activeTargets.includes(props.to)" :to="'#portal-' + props.to">
    <slot />
  </Teleport>
</template>
