<script setup lang="ts">
const { title, noSpacer } = $(withDefaults(defineProps<{
  title?: string,
  noSpacer?: boolean,
}>(), {
  title: "",
  noSpacer: false,
}));
watchEffect(() => useTitle(`${title} - nfSense`));
</script>
<template>
  <Portal to="page-header">
    <h1 v-if="title !== ''" v-text="title" :class="{'flex-grow': !noSpacer}"/>
    <slot/>
  </Portal>
</template>
