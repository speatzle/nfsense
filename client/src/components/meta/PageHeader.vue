<script setup lang="ts">
const props = withDefaults(defineProps<{
  title?: string,
  noSpacer?: boolean,
}>(), {
  title: '',
  noSpacer: false,
});
const { title, noSpacer } = $(props);
watchEffect(() => useTitle(`${title} - nfSense`));
</script>
<template>
  <Portal to="page-header">
    <h1 v-if="title !== ''" :class="{'flex-grow': !noSpacer}" v-text="title"/>
    <slot/>
  </Portal>
</template>
