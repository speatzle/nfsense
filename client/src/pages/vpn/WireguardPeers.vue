<script setup lang="ts">
import { apiCall } from "../../api";
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
  let res = await apiCall("VPN.GetWireguardPeers", {});
  if (res.Error === null) {
    console.debug("peers", res.Data.WireguardPeers);
    peers = res.Data.WireguardPeers;
  } else {
    console.debug("error", res);
  }
  loading = false;
}

async function deletePeer(){
  let res = await apiCall("VPN.DeleteWireguardPeer", {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug("deleted peer");
  } else {
    console.debug("error", res);
  }
  load();
}

async function editPeer() {
  p.router.push("/vpn/wireguardpeers/edit/" + displayData[selection[0]].name);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Peers" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/vpn/wireguardpeers/edit">Create</router-link>
    <button @click="editPeer" :disabled="selection.length != 1">Edit</button>
    <button @click="deletePeer" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>