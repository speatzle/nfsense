<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let services = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Type', path: 'type' },
  { heading: 'Value', path: 'value' },
  { heading: 'Comment', path: 'comment' },
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const index in services) {
    data.push({
      name: services[index].name,
      value: getServiceValue(services[index]),
      type: Object.keys(services[index].service_type)[0],
      comment: services[index].comment,
    });
  }
  return data;
});

function getServiceValue(s: any): string {
  console.debug('test', Object.keys(s.service_type)[0]);
  let value: string;
  switch (Object.keys(s.service_type)[0]) {
  case 'tcp':
  case 'udp':
    value = getServicePortRange(s.service_type[Object.keys(s.service_type)[0]]);
    break;
  case 'icmp':
    value = 'icmp';
    break;
  case 'group':
    value = s.children;
    break;
  default:
    value = 'unkown';
  }
  return value;
}

function getServicePortRange(s:any): string {
  return `${s.source}-${s.destination}`;
}

async function load(){
  loading = true;
  let res = await apiCall('object.services.list', {});
  if (res.Error === null) {
    console.debug('services', res.Data);
    services = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteService(){
  let res = await apiCall('object.services.delete', { name: displayData[selection[0]].name });
  if (res.Error === null) {
    console.debug('deleted service');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editService() {
  p.router.push(`/object/services/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="displayData" title="Services" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/object/services/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editService">Edit</button>
    <button :disabled="selection.length != 1" @click="deleteService">Delete</button>
  </TableView>
</template>