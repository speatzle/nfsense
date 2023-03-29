<script setup lang="ts">

const props = defineModel<{
  title: string,
  loadData: () => void,
  columns?: {
    heading: string,
    path: string,
    component?: Component,
  }[],
  data: Record<string, any>[],
  tableProps: any,
}>();

let { title, columns, loadData, data, tableProps } = $(props);

let loading = $ref(true);

async function load() {
  console.debug("Start loading...");
  loading = true;
  loadData();
  loading = false;
  console.debug("Finished loading");
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
    <PageHeader :title="title">
      <button @click="load">Load</button>
    </PageHeader>
    <div v-if="loading" >Loading...</div>
    <NiceTable v-else :columns="columns" v-bind="tableProps" :data="data"/>
  </div>
</template>