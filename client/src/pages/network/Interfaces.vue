<script setup lang="ts">
import { apiCall } from "../../api";

let interfaces = $ref({});
const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Members', path: 'members'},
  {heading: 'Addressing Mode', path: 'addressing_mode'},
  {heading: 'Address', path: 'address'},
];

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in interfaces) {
    data.push({
      name,
      type: interfaces[name].type,
      addressing_mode: interfaces[name].addressing_mode,
      address: interfaces[name].address,
      comment: interfaces[name].comment,
    });
  }
  return data;
}); 

function getServicePortRange(s:any): string {
  if (s.dport_end) {
    return s.dport_start + "-" + s.dport_end;
  }
  return s.dport_start;
}

async function load(){
  let res = await apiCall("Network.GetInterfaces", {});
  if (res.Error === null) {
    console.debug("interfaces", res.Data.Interfaces);
    interfaces = res.Data.Interfaces;
  } else {
    console.debug("error", res);
  }
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Interfaces" :columns="columns" :load-data="load" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}"/>
</template>