<script setup lang="ts">
import { apiCall } from "../../api";
import getPlugins from '../../plugins';
const p = getPlugins();

let addresses = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Value', path: 'value'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  loading = true;
  let res = await apiCall("Object.GetAddresses", {});
  if (res.Error === null) {
    addresses = res.Data.Addresses;
    console.debug("addresses", addresses);
  } else {
    console.debug("error", res);
  }
  loading = false;
}

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in addresses) {
    data.push({
      name,
      value: getAddressValue(addresses[name]),
      type: addresses[name].type,
      comment: addresses[name].comment,
    });
  }
  return data;
});

function getAddressValue(s: any): string {
  let value: string;
  switch (s.type) {
  case "host":
    value = s.host;
    break;
  case "range":
    value = s.range;
    break;
  case "network":
    value = s.network;
    break;
  case "group":
    value = s.children;
    break;
  default:
    value = "unkown";
  }
  return value;
}

async function deleteAddress(){
  let res = await apiCall("Object.DeleteAddress", {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug("deleted address");
  } else {
    console.debug("error", res);
  }
  load();
}

async function editInterface() {
  p.router.push("/object/addresses/edit/" + displayData[selection[0]].name);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Addresses" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/object/addresses/edit">Create</router-link>
    <button @click="editInterface" :disabled="selection.length != 1">Edit</button>
    <button @click="deleteAddress" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>