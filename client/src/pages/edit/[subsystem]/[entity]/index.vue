<script setup lang="ts">
const props = $defineProps<{entity: string}>();
const { entity } = $(props);

async function create() {
  
}

</script>
<template>
  <div>
    <PageHeader title="Create TODO">
      <button @click="create">Create</button>
      <button @click="$router.go(-1)">Discard</button>
    </PageHeader>
  </div>
</template>

<style scoped>
</style>