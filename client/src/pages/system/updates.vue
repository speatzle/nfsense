<script setup lang="ts">
import { apiCall } from '../../api';

type UpdateStatus = any

let updateStatus: UpdateStatus[] = $ref([]);
let loading = $ref(false);

async function load(){
  loading = true;
  const res = await apiCall('system.update.status', {});
  if (res.Error === null) {
    console.debug('update status', res.Data);
    updateStatus = res.Data;
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
        <p>{{ updateStatus }}</p>
    </template>
    <div v-else>
      Loading...
    </div>
  </div>
</template>

<style scoped>
</style>
