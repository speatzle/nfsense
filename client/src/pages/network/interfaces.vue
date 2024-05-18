<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let interfaces = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Members', path: 'members'},
  {heading: 'Addressing Mode', path: 'addressing_mode'},
  {heading: 'Address', path: 'address'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const index in interfaces) {
    data.push({
      name: interfaces[index].name,
      type: interfaces[index].type,
      addressing_mode: interfaces[index].addressing_mode,
      address: interfaces[index].address,
      comment: interfaces[index].comment,
    });
  }
  return data;
});

async function load(){
  loading = true;
  let res = await apiCall('network.interfaces.list', {});
  if (res.Error === null) {
    console.debug('interfaces', res.Data);
    interfaces = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteInterface(){
  let res = await apiCall('network.interfaces.delete', {name: displayData[selection[0]].name});
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