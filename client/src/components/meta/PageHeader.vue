<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    title?: string;
    noSpacer?: boolean;
  }>(),
  {
    title: "",
    noSpacer: false,
  },
);
watchEffect(() => (document.title = `${props.title} - nfSense`));
onBeforeUnmount(() => (document.title = "nfSense"));
</script>
<template>
  <Portal to="page-header">
    <TransitionGroup name="fade" appear>
      <h1
        v-if="props.title !== ''"
        :class="{ 'flex-grow': !props.noSpacer }"
        v-text="props.title"
      />
      <slot />
    </TransitionGroup>
  </Portal>
</template>
