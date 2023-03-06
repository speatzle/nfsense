<script lang="ts">
let activeTargets = $ref<string[]>([]);
</script>

<script setup lang="ts">
const props = $defineProps<{
  from?: string,
  to?: string,
}>();
const { from, to } = $(props);

if (from) {
  onMounted(() => activeTargets.push(from));
  onBeforeUnmount(() => activeTargets.splice(activeTargets.indexOf(from), 1));
}
</script>

<template>
  <div v-if="from" :id="'portal-' + from">
    <slot/>
  </div>
  <Teleport v-else-if="to && activeTargets.includes(to)" :to="'#portal-' + to">
    <slot/>
  </Teleport>
</template>
