<script setup lang="ts">
import { apiCall } from '../../api';

let staticRoutes = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Interface', path: 'interface' },
  { heading: 'Gateway', path: 'gateway' },
  { heading: 'Destination', path: 'destination' },
  { heading: 'Metric', path: 'metric' },
];

async function load(){
  loading = true;
  let res = await apiCall('network.static_routes.list', {});
  if (res.Error === null) {
    console.debug('staticRoutes', res.Data);
    staticRoutes = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteStaticRoutes(){
  let res = await apiCall('network.static_routes.delete', { index: selection[0] });
  if (res.Error === null) {
    console.debug('deleted static routes');
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
  <TableView v-model:selection="selection" v-model:data="staticRoutes" title="Static Routes" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/network/static_routes/edit">Create</router-link>
    <router-link class="button" :class="{ disabled: selection.length != 1 }" :to="'/network/static_routes/edit/' + selection[0]">Edit</router-link>
    <button :disabled="selection.length != 1" @click="deleteStaticRoutes">Delete</button>
  </TableView>
</template>
