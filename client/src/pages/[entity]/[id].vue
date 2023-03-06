<script setup lang="ts">
const props = $defineProps<{entity: string, id: string}>();
const { entity, id } = $(props);

const pageTypes: { [key: string]: any } = {
  "rules": { title: "Rules" },
  "addresses": { title: "Addresses"},
};
</script>
<template>
  <div>
    <PageHeader :title="pageTypes[entity].title"/>
    {{ entity }} {{ id }}
  </div>
</template>
