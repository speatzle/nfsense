<script setup lang="ts">
import { apiCall } from "../../api";

let rules = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Source', path: 'match.source_addresses'},
  {heading: 'Destination', path: 'match.destination_addresses'},
  {heading: 'Service', path: 'match.services'},
  {heading: 'Verdict', path: 'verdict'},
  {heading: 'Counter', path: 'counter'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  let res = await apiCall("Firewall.GetForwardRules", {});
  if (res.Error === null) {
    rules = res.Data.forward_rules;
    console.debug("rules", rules);
  } else {
    console.debug("error", res);
  }
}

async function deleteRule(){
  let res = await apiCall("Firewall.DeleteForwardRule", {index: selection[0]});
  if (res.Error === null) {
    console.debug("deleted rule");
  } else {
    console.debug("error", res);
  }
  load();
}

async function draggedRow(draggedRow: number, draggedOverRow: number) {
  console.log("dragged", draggedRow, draggedOverRow);
  let res = await apiCall("Firewall.MoveForwardRule", {index: draggedRow, to_index: draggedOverRow});
  if (res.Error === null) {
    console.debug("deleted rule");
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
      <TableView title="Forward Rules" :columns="columns" :loading="loading" @draggedRow="draggedRow" v-model:selection="selection" v-model:data="rules" :table-props="{sort:true, sortSelf: true, draggable: true}">
      <button @click="load">Refresh</button>
      <router-link class="button" to="/edit/firewall/forwardrules">Create</router-link>
      <router-link class="button" :disabled="selection.length != 1" :to="'/edit/firewall/forwardrules/' + selection[0]">Edit</router-link>
      <button @click="deleteRule" :disabled="selection.length != 1">Delete</button>
    </TableView>
  </div>
</template>