<script setup lang="ts">
import { apiCall } from "../../api";

let services = $ref({});
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
  let res = await apiCall("Object.GetServices", {});
  if (res.Error === null) {
    console.debug("services", res.Data.Services);
    services = res.Data.Services;
  } else {
    console.debug("error", res);
  }
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Services" :columns="columns" :load-data="load" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}"/>
</template>