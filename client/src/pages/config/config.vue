<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();


let changelog = $ref([]);
let loading = $ref(false);

const columns = [
  {heading: 'Path', path: 'path'},
  {heading: 'Action', path: 'action'},
  {heading: 'ID', path: 'id'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const change of changelog) {
    data.push({
      path: change.path,
      action: change.action,
      id: change.id,
    });
  }
  return data;
});

async function load(){
  loading = true;
  let res = await apiCall('config.get_pending_changelog', {});
  if (res.Error === null) {
    console.debug('changelog', res.Data);
    changelog = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function apply(){
  let res = await apiCall('config.apply_pending_changes', {});
  if (res.Error === null) {
    console.debug('apply');
    p.toast.success('Applied Pending Config');
  } else {
    console.debug('error', res);
  }
  load();
}

async function discard(){
  let res = await apiCall('config.discard_pending_changes', {});
  if (res.Error === null) {
    console.debug('discard');
    p.toast.success('Discarded Pending Config');
  } else {
    console.debug('error', res);
  }
  load();
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Pending Changes" :columns="columns" :loading="loading" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <button @click="apply">Apply</button>
    <button @click="discard">Discard</button>
  </TableView>
</template>