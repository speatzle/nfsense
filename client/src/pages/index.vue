<script setup lang="ts">
import { apiCall } from '../api';

type Link = {
  name: string,
  carrier_state: string,
  operational_state: string,
}

let links: Link[] = $ref([]);
let loading = $ref(false);

async function load(){
  loading = true;
  let res = await apiCall('network.links.get', {});
  if (res.Error === null) {
    console.debug('links', res.Data);
    links = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div style="overflow-y: auto;">
    <PageHeader title="Dashboard">
    </PageHeader>
    <template v-if="!loading">
      <div v-for="(link, index) in links" :key="index">
        <p>{{ link.name }} {{ link.carrier_state }} {{ link.operational_state }}</p>
      </div>
    </template>
    <div v-else>
      Loading...
    </div>
  </div>
</template>

<style scoped>
</style>