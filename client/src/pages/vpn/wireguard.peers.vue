<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let peers = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Allowed IPs', path: 'allowed_ips'},
  {heading: 'Endpoint', path: 'endpoint'},
  {heading: 'Persistent Keepalive', path: 'persistent_keepalive'},
  {heading: 'Comment', path: 'comment'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in peers) {
    data.push({
      name,
      allowed_ips: peers[name].allowed_ips,
      endpoint: peers[name].endpoint,
      persistent_keepalive: peers[name].persistent_keepalive,
      comment: peers[name].comment,
    });
  }
  return data;
});

async function load(){
  loading = true;
  let res = await apiCall('vpn.wireguard.peers.list', {});
  if (res.Error === null) {
    console.debug('peers', res.Data);
    peers = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deletePeer(){
  let res = await apiCall('vpn.wireguard.peers.delete', {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug('deleted peer');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editPeer() {
  p.router.push(`/vpn/wireguard.peers/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="displayData" title="Peers" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/vpn/wireguard.peers/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editPeer">Edit</button>
    <button :disabled="selection.length != 1" @click="deletePeer">Delete</button>
  </TableView>
</template>