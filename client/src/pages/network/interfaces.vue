<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
import EnumTypeDisplay from '~/components/display/EnumTypeDisplay.vue';
const p = getPlugins();

let interfaces = $ref({} as any); // TODO: Add proper type
let loading = $ref(false);
const selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Alias', path: 'alias' },
  { heading: 'Type', path: 'interface_type', component: markRaw(EnumTypeDisplay), componentProp: 'data' },
  { heading: 'Addressing Mode', path: 'addressing_mode', component: markRaw(EnumTypeDisplay), componentProp: 'data' },
  { heading: 'Address', path: 'addressing_mode.static.address' },
  { heading: 'Vlan Parent', path: 'interface_type.vlan.parent' },
  { heading: 'Vlan ID', path: 'interface_type.vlan.id' },
  { heading: 'Bond Members', path: 'interface_type.bond.members', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Bridge Members', path: 'interface_type.bridge.members', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Comment', path: 'comment' },
];

const displayData = $computed(() => {
  const data = [];
  for (const index in interfaces) {
    data.push({
      name: interfaces[index].name,
      alias: interfaces[index].alias,
      interface_type: interfaces[index].interface_type,
      addressing_mode: interfaces[index].addressing_mode,
      address: interfaces[index].address,
      comment: interfaces[index].comment,
    });
  }
  return data;
});

async function load(){
  loading = true;
  const res = await apiCall('network.interfaces.list', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    interfaces = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteInterface(){
  const res = await apiCall('network.interfaces.delete', { name: displayData[selection[0]].name });
  if (res.Error === null) {
    console.debug('deleted interface');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editInterface() {
  p.router.push(`/network/interfaces/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="displayData" title="Interfaces" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/network/interfaces/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editInterface">Edit</button>
    <button :disabled="selection.length != 1" @click="deleteInterface">Delete</button>
  </TableView>
</template>
