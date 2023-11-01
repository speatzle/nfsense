<script setup lang="ts">
import { apiCall } from '../../api';

let status = $ref('');
let loading = $ref(false);

async function load() {
  loading = true;
  let res = await apiCall('vpn.wireguard.get_status', {});
  if (res.Error === null) {
    console.debug('status', res.Data);
    status = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

onMounted(async () => {
  load();
});

</script>

<template>
  <div style="overflow-y: auto;">
    <PageHeader title="Wireguard Status">
    </PageHeader>
    <template v-if="!loading">
      <div v-for="(line, index) in status.split('\n')" :key="index">
        <p>{{ line }}</p>
      </div>
    </template>
    <div v-else>
      Loading...
    </div>
  </div>
</template>

<style scoped></style>