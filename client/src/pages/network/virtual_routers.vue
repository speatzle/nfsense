<script setup lang="ts">
import { apiCall } from '../../api';

let staticRoutes = $ref([]);
let loading = $ref(false);
const selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Table ID', path: 'table_id' },
  { heading: 'Comment', path: 'comment' },
];

async function load(){
  loading = true;
  const res = await apiCall('network.virtual_routers.list', {});
  if (res.Error === null) {
    console.debug('staticRoutes', res.Data);
    staticRoutes = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteVirtualRouter(){
  const res = await apiCall('network.virtual_routers.delete', { index: selection[0] });
  if (res.Error === null) {
    console.debug('deleted virtual router');
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
  <TableView v-model:selection="selection" v-model:data="staticRoutes" title="Virtual Router" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/network/virtual_routers/edit">Create</router-link>
    <router-link class="button" :class="{ disabled: selection.length != 1 }" :to="'/network/virtual_routers/edit/' + selection[0]">Edit</router-link>
    <button :disabled="selection.length != 1" @click="deleteVirtualRouter">Delete</button>
  </TableView>
</template>
