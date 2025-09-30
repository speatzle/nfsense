<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import EnumValueDisplay from '~/components/display/EnumValueDisplay.vue';
import EnumTypeDisplay from '~/components/display/EnumTypeDisplay.vue';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
const p = getPlugins();

let addresses = $ref([] as any[]); // TODO: Add proper type
let loading = $ref(false);
const selection = $ref([] as number[]);

const addressValueDefinition: { [key: string]: { path: string, component: Component | undefined } } = {
  'host': { path: 'host.address', component: undefined },
  'range': { path: 'range.address', component: undefined },
  'network': { path: 'network.network', component: undefined },
  'group': { path: 'group.members', component: ArrayDisplay },
};


const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Type', path: 'address_type', component: markRaw(EnumTypeDisplay), componentProp: 'data' },
  { heading: 'Value', path: 'address_type', component: markRaw(EnumValueDisplay), componentProp: 'data', props: { definition: addressValueDefinition } },
  { heading: 'Comment', path: 'comment' },
];

async function load(){
  loading = true;
  const res = await apiCall('object.addresses.list', {});
  if (res.Error === null) {
    addresses = res.Data;
    console.debug('addresses', addresses);
  } else {
    console.debug('error', res);
  }
  loading = false;
}

const displayData = $computed(() => {
  const data = [];
  for (const index in addresses) {
    data.push({
      name: addresses[index].name,
      address_type: addresses[index].address_type,
      comment: addresses[index].comment,
    });
  }
  return data;
});

async function deleteAddress(){
  const res = await apiCall('object.addresses_delete', { id: displayData[selection[0]].name });
  if (res.Error === null) {
    console.debug('deleted address');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editAddress() {
  p.router.push(`/object/addresses/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="displayData" title="Addresses" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/object/addresses/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editAddress">Edit</button>
    <button :disabled="selection.length != 1" @click="deleteAddress">Delete</button>
  </TableView>
</template>
