<script setup lang="ts">
import { apiCall } from '../api';

type ServiceStatus = {
  name: string,
  status: string,
}

let serviceStatus: ServiceStatus[] = $ref([]);
let loading = $ref(false);

async function load(){
  loading = true;
  let res = await apiCall('system.services.status', {});
  if (res.Error === null) {
    console.debug('services staus', res.Data);
    serviceStatus = res.Data;
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
      <div v-for="(status, index) in serviceStatus" :key="index">
        <p>{{ status.name }}: {{ status.status }}</p>
      </div>
    </template>
    <div v-else>
      Loading...
    </div>
  </div>
</template>

<style scoped>
</style>
