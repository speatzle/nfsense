<script setup lang="ts">
import { apiCall } from "../../api";

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

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Interfaces" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Apply</button>
    <button @click="load">Refresh</button>
    <button @click="load">Create</button>
    <button @click="load" :disabled="selection.length != 1">Edit</button>
    <button @click="load" :disabled="selection.length == 0">Delete</button>
  </TableView>
</template>