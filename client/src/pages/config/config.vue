<script setup lang="ts">
import { apiCall } from "../../api";

let changelog = $ref([]);
let loading = $ref(false);

const columns = [
  {heading: 'Path', path: 'path'},
  {heading: 'Type', path: 'type'},
  {heading: 'From', path: 'from'},
  {heading: 'To', path: 'to'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const change of changelog) {
    data.push({
      path: change.path,
      type: change.type,
      from: change.from,
      to: change.to,
    });
  }
  return data;
});

async function load(){
  loading = true
  let res = await apiCall("Config.GetPendingChangelog", {});
  if (res.Error === null) {
    console.debug("changelog", res.Data.Changelog);
    changelog = res.Data.Changelog;
  } else {
    console.debug("error", res);
  }
  loading = false
}

async function apply(){
  let res = await apiCall("Config.ApplyPendingChanges", {});
  if (res.Error === null) {
    console.debug("apply");
  } else {
    console.debug("error", res);
  }
  load()
}

async function discard(){
  let res = await apiCall("Config.DiscardPendingChanges", {});
  if (res.Error === null) {
    console.debug("apply");
  } else {
    console.debug("error", res);
  }
  load()
}

onMounted(async() => {
  load();
});

</script>

<template>
    <TableView title="Pending Changes" :columns="columns" :loading="loading" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
        <button @click="load">Refresh</button>
        <button @click="apply">Apply</button>
        <button @click="discard">Discard</button>
    </TableView>
</template>