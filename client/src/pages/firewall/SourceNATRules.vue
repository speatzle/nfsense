<script setup lang="ts">
import { apiCall } from "../../api";

let rules = $ref([]);
const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Source', path: 'match.source_addresses'},
  {heading: 'Destination', path: 'match.destination_addresses'},
  {heading: 'Service', path: 'match.services'},
  {heading: 'Counter', path: 'counter'},
  {heading: 'Comment', path: 'comment'},
];

async function loadRules(){
  let res = await apiCall("Firewall.GetSourceNATRules", {});
  if (res.Error === null) {
    rules = res.Data.SourceNATRules;
    console.debug("rules", rules);
  } else {
    console.debug("error", res);
  }
}

onMounted(async() => {
  loadRules();
});

</script>

<template>
  <div>
    <PageHeader title="SNAT Rules">
      <button @click="loadRules">Load Rules</button>
    </PageHeader>
    <NiceTable :columns="columns" v-model:data="rules" draggable="true"/>
  </div>
</template>