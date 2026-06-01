<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    title?: string;
  }>(),
  { title: "" },
);
watchEffect(() => (document.title = `${props.title} - nfSense`));
onBeforeUnmount(() => (document.title = "nfSense"));
</script>
<template>
  <div class="page pad gap">
    <div class="page-header gap">
      <h1 v-if="props.title !== ''" class="flex-grow" v-text="props.title" />
      <slot name="header" />
    </div>
    <div class="page-content gap">
      <slot />
    </div>
  </div>
</template>
<style scoped>
.page {
  overflow: visible;
  grid-area: PC;
  display: grid;
  grid-template-rows: auto 1fr;

  & > .page-header {
    flex-flow: row nowrap;
    align-items: center;
    overflow-x: auto;
    overflow-y: hidden;

    & button svg {
      margin: -0.25rem;
    }
  }
  & > .page-content {
    overflow: auto;
  }
}
</style>
