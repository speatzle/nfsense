<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import EnumTypeDisplay from '~/components/display/EnumTypeDisplay.vue';
import EnumValueDisplay from '~/components/display/EnumValueDisplay.vue';
import PortServiceDisplay from '~/components/display/PortServiceDisplay.vue';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
const p = getPlugins();

let services = $ref({});
let loading = $ref(false);
let selection = $ref([] as number[]);

const serviceValueDefinition: { [key: string]: { path: string, component: Component | undefined } } = {
  'tcp': { path: 'tcp', component: PortServiceDisplay },
  'udp': { path: 'udp', component: PortServiceDisplay },
  'icmp': { path: 'icmp.code', component: undefined },
  'group': { path: 'group.members', component: ArrayDisplay },
};

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Type', path: 'service_type', component: markRaw(EnumTypeDisplay), componentProp: 'data', props: { definition: serviceValueDefinition } },
  { heading: 'Value', path: 'service_type', component: markRaw(EnumValueDisplay), componentProp: 'data', props: { definition: serviceValueDefinition } },
  { heading: 'Comment', path: 'comment' },
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const index in services) {
    data.push({
      name: services[index].name,
      service_type: services[index].service_type,
      comment: services[index].comment,
    });
  }
  return data;
});

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
