<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let servers = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  { heading: 'Interface', path: 'interface' },
  { heading: 'Comment', path: 'comment' },
];

async function load(){
  let res = await apiCall('service.dns_servers.list', {});
  if (res.Error === null) {
    servers = res.Data;
    console.debug('rules', servers);
  } else {
    console.debug('error', res);
  }
}

async function deleteRule(){
  let res = await apiCall('service.dns_servers.delete', { index: selection[0] });
  if (res.Error === null) {
    console.debug('deleted server');
    p.toast.success('Deleted DNS Server');
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
  <div>
    <TableView v-model:selection="selection" v-model:data="servers" title="DNS Servers" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
      <button @click="load">Refresh</button>
      <router-link class="button" to="/service/dns_servers/edit">Create</router-link>
      <router-link class="button" :class="{ disabled: selection.length != 1 }" :to="'/service/dns_servers/edit/' + selection[0]">Edit</router-link>
      <button :disabled="selection.length != 1" @click="deleteRule">Delete</button>
    </TableView>
  </div>
</template>