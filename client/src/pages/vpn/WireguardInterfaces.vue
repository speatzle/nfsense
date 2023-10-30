<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let interfaces = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Listen Port', path: 'listen_port'},
  {heading: 'Peers', path: 'peers'},
  {heading: 'Comment', path: 'comment'},

];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in interfaces) {
    data.push({
      name,
      listen_port: interfaces[name].listen_port,
      peers: interfaces[name].peers,
      comment: interfaces[name].comment,
    });
  }
  return data;
});

async function load(){
  loading = true;
  let res = await apiCall('vpn.wireguard.get_interfaces', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    interfaces = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteInterface(){
  let res = await apiCall('vpn.wireguard_delete_interface', {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug('deleted interface');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editInterface() {
  p.router.push(`/vpn/wireguardinterfaces/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Wireguard Interfaces" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/vpn/wireguardinterfaces/edit">Create</router-link>
    <button @click="editInterface" :disabled="selection.length != 1">Edit</button>
    <button @click="deleteInterface" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>