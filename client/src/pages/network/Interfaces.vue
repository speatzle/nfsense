<script setup lang="ts">
import { apiCall } from "../../api";
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
  for (const name in interfaces) {
    data.push({
      name,
      type: interfaces[name].type,
      addressing_mode: interfaces[name].addressing_mode,
      address: interfaces[name].address,
      comment: interfaces[name].comment,
    });
  }
  return data;
});

async function load(){
  loading = true
  let res = await apiCall("Network.GetInterfaces", {});
  if (res.Error === null) {
    console.debug("interfaces", res.Data.Interfaces);
    interfaces = res.Data.Interfaces;
  } else {
    console.debug("error", res);
  }
  loading = false
}

async function deleteInterface(){
  let res = await apiCall("Network.DeleteInterface", {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug("deleted interface");
  } else {
    console.debug("error", res);
  }
  load();
}

async function editInterface() {
  p.router.push("/network/interfaces/edit/" + displayData[selection[0]].name)
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Interfaces" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/network/interfaces/edit">Create</router-link>
    <button @click="editInterface" :disabled="selection.length != 1">Edit</button>
    <button @click="deleteInterface" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>