<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
const p = getPlugins();

let interfaces = $ref({} as any); // TODO: Add proper type
let loading = $ref(false);
const selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Listen Port', path: 'listen_port' },
  { heading: 'Peers', path: 'peers', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Comment', path: 'comment' },

];

const displayData = $computed(() => {
  const data = [];
  for (const index in interfaces) {
    data.push({
      name: interfaces[index].name,
      listen_port: interfaces[index].listen_port,
      peers: interfaces[index].peers,
      comment: interfaces[index].comment,
    });
  }
  return data;
});

async function load(){
  loading = true;
  const res = await apiCall('vpn.wireguard.interfaces.list', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    interfaces = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function generateKeys(){
  const res = await apiCall('vpn.wireguard.keypair.generate', {});
  if (res.Error === null) {
    console.debug('keypair', res.Data);
    alert(JSON.stringify(res.Data));
  } else {
    console.debug('error', res);
  }
}

async function deleteInterface(){
  const res = await apiCall('vpn.wireguard.interfaces.delete', { name: displayData[selection[0]].name });
  if (res.Error === null) {
    console.debug('deleted interface');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editInterface() {
  p.router.push(`/vpn/wireguard.interfaces/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="displayData" title="Wireguard Interfaces" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <button @click="generateKeys">Generate Keys</button>
    <router-link class="button" to="/vpn/wireguard.interfaces/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editInterface">Edit</button>
    <button :disabled="selection.length != 1" @click="deleteInterface">Delete</button>
  </TableView>
</template>
