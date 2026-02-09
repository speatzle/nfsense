<script setup lang="ts">
import { apiCall } from '../api';
import getPlugins from '../plugins';
const p = getPlugins();

type ServiceStatus = {
  name: string,
  status: string,
}

let serviceStatus: ServiceStatus[] = $ref([]);
let loading = $ref(false);

async function load(){
  loading = true;
  const res = await apiCall('system.services.status', {});
  if (res.Error === null) {
    console.debug('services staus', res.Data);
    serviceStatus = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function restart(){
  const res = await apiCall('system.power.restart', {});
  if (res.Error === null) {
    p.toast.success('Restart Triggered');
  } else {
    console.debug('error', res);
  }
}

async function shutdown(){
  const res = await apiCall('system.power.shutdown', {});
  if (res.Error === null) {
    p.toast.success('Shutdown Triggered');
  } else {
    console.debug('error', res);
  }
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
      <button @click="shutdown">Shutdown</button>
      <button @click="restart">Restart</button>
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
