<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
import ArrayDisplay from '~/components/display/ArrayDisplay.vue';
const p = getPlugins();

let rules = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Source', path: 'source_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Destination', path: 'destination_addresses', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Service', path: 'services', component: markRaw(ArrayDisplay), componentProp: 'data' },
  { heading: 'Verdict', path: 'verdict' },
  { heading: 'Counter', path: 'counter' },
  { heading: 'Comment', path: 'comment' },
];

async function load(){
  let res = await apiCall('firewall.forward_rules.list', {});
  if (res.Error === null) {
    rules = res.Data;
    console.debug('rules', rules);
  } else {
    console.debug('error', res);
  }
}

async function deleteRule(){
  let res = await apiCall('firewall.forward_rules.delete', { id: selection[0] });
  if (res.Error === null) {
    console.debug('deleted rule');
    p.toast.success('Deleted Rule');
  } else {
    console.debug('error', res);
  }
  load();
}

async function draggedRow(draggedRow: number, draggedOverRow: number) {
  console.log('dragged', draggedRow, draggedOverRow);
  let res = await apiCall('firewall.forward_rules.move', { index: draggedRow, to_index: draggedOverRow });
  if (res.Error === null) {
    console.debug('moved rule');
    p.toast.success('Moved Rule');
  } else {
    console.debug('error', res);
  }
  load();
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
    <TableView v-model:selection="selection" v-model:data="rules" title="Forward Rules" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true, draggable: true}" @dragged-row="draggedRow">
      <button @click="load">Refresh</button>
      <router-link class="button" to="/firewall/forward_rules/edit">Create</router-link>
      <router-link class="button" :class="{ disabled: selection.length != 1 }" :to="'/firewall/forward_rules/edit/' + selection[0]">Edit</router-link>
      <button :disabled="selection.length != 1" @click="deleteRule">Delete</button>
    </TableView>
  </div>
</template>