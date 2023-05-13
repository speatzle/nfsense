<script setup lang="ts">
import { apiCall } from "../../api";
import getPlugins from '../../plugins';
const p = getPlugins();

let services = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Value', path: 'value'},
  {heading: 'Comment', path: 'comment'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in services) {
    data.push({
      name,
      value: getServiceValue(services[name]),
      type: services[name].type,
      comment: services[name].comment,
    });
  }
  return data;
});

function getServiceValue(s: any): string {
  let value: string;
  switch (s.type) {
  case "tcp":
  case "udp":
    value = getServicePortRange(s);
    break;
  case "icmp":
    value = "icmp";
    break;
  case "group":
    value = s.children;
    break;
  default:
    value = "unkown";
  }
  return value;
}

function getServicePortRange(s:any): string {
  if (s.dport_end) {
    return s.dport_start + "-" + s.dport_end;
  }
  return s.dport_start;
}

async function load(){
  loading = true;
  let res = await apiCall("Object.GetServices", {});
  if (res.Error === null) {
    console.debug("services", res.Data.Services);
    services = res.Data.Services;
  } else {
    console.debug("error", res);
  }
  loading = false;
}

async function deleteService(){
  let res = await apiCall("Object.DeleteService", {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug("deleted service");
  } else {
    console.debug("error", res);
  }
  load();
}

async function editService() {
  p.router.push("/object/services/edit/" + displayData[selection[0]].name);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Services" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/object/services/edit">Create</router-link>
    <button @click="editService" :disabled="selection.length != 1">Edit</button>
    <button @click="deleteService" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>