<script setup lang="ts">
import { apiCall } from "../../api";
import getPlugins from '../../plugins';
const p = getPlugins();

let rules = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Source', path: 'match.source_addresses'},
  {heading: 'Destination', path: 'match.destination_addresses'},
  {heading: 'Service', path: 'match.services'},
  {heading: 'Counter', path: 'counter'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  let res = await apiCall("Firewall.GetSourceNATRules", {});
  if (res.Error === null) {
    rules = res.Data.source_nat_rules;
    console.debug("rules", rules);
  } else {
    console.debug("error", res);
  }
}

async function deleteRule(){
  let res = await apiCall("Firewall.DeleteSourceNATRule", {index: selection[0]});
  if (res.Error === null) {
    console.debug("deleted rule");
    p.toast.success("Deleted Rule");
  } else {
    console.debug("error", res);
  }
  load();
}

async function draggedRow(draggedRow: number, draggedOverRow: number) {
  console.log("dragged", draggedRow, draggedOverRow);
  let res = await apiCall("Firewall.MoveSourceNATRule", {index: draggedRow, to_index: draggedOverRow});
  if (res.Error === null) {
    console.debug("moved rule");
    p.toast.success("Moved Rule");
  } else {
    console.debug("error", res);
  }
  load();
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
      <TableView title="SNAT Rules" :columns="columns" :loading="loading" @draggedRow="draggedRow" v-model:selection="selection" v-model:data="rules" :table-props="{sort:true, sortSelf: true, draggable: true}">
      <button @click="load">Refresh</button>
      <router-link class="button" to="/firewall/sourcenatrules/edit">Create</router-link>
      <router-link class="button" :class="{ disabled: selection.length != 1 }" :to="'/firewall/sourcenatrules/edit/' + selection[0]">Edit</router-link>
      <button @click="deleteRule" :disabled="selection.length != 1">Delete</button>
    </TableView>
  </div>
</template>