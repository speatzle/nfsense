<script setup lang="ts">
import { apiCall } from "../../api";

let addresses = $ref([]);
const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Value', path: 'value'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  let res = await apiCall("Object.GetAddresses", {});
  if (res.Error === null) {
    addresses = res.Data.Addresses;
    console.debug("addresses", addresses);
  } else {
    console.debug("error", res);
  }
}

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in addresses) {
    data.push({
      name,
      value: getAddressValue(addresses[name]),
      type: addresses[name].type,
      comment: addresses[name].comment,
    });
  }
  return data;
});

function getAddressValue(s: any): string {
  let value: string;
  switch (s.type) {
  case "host":
    value = s.host;
    break;
  case "range":
    value = s.range;
    break;
  case "network":
    value = s.network;
    break;
  case "group":
    value = s.children;
    break;
  default:
    value = "unkown";
  }
  return value;
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
    <PageHeader title="Addresses">
      <button @click="load">Load Addresses</button>
    </PageHeader>
    <NiceTable :columns="columns" v-model:data="displayData"/>
  </div>
</template>