<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
const p = getPlugins();

let peers = $ref({} as any); // TODO: Add proper type
let loading = $ref(false);
const selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Allowed IPs', path: 'allowed_ips', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Endpoint', path: 'endpoint' },
  { heading: 'Persistent Keepalive', path: 'persistent_keepalive' },
  { heading: 'Comment', path: 'comment' },
];

const displayData = $computed(() => {
  const data = [];
  for (const index in peers) {
    data.push({
      name: peers[index].name,
      allowed_ips: peers[index].allowed_ips,
      endpoint: peers[index].endpoint,
      persistent_keepalive: peers[index].persistent_keepalive,
      comment: peers[index].comment,
    });
  }
  return data;
});

async function load() {
  loading = true;
  const res = await apiCall('vpn.wireguard.peers.list', {});
  if (res.Error === null) {
    console.debug('peers', res.Data);
    peers = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deletePeer() {
  const res = await apiCall('vpn.wireguard.peers.delete', {
    name: displayData[selection[0]].name,
  });
  if (res.Error === null) {
    console.debug('deleted peer');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editPeer() {
  p.router.push(
    `/vpn/wireguard.peers/edit/${displayData[selection[0]].name}`,
  );
}

onMounted(async () => {
  load();
});
</script>

<template>
  <TableView
    v-model:selection="selection"
    v-model:data="displayData"
    title="Peers"
    :columns="columns"
    :loading="loading"
    :table-props="{ sort: true, sortSelf: true }"
  >
    <button @click="load">Refresh</button>
    <router-link class="button" to="/vpn/wireguard.peers/edit"
    >
      Create
    </router-link
    >
    <button :disabled="selection.length != 1" @click="editPeer">
      Edit
    </button>
    <button :disabled="selection.length != 1" @click="deletePeer">
      Delete
    </button>
  </TableView>
</template>
