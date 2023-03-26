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

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
    <PageHeader title="Addresses">
      <button @click="load">Load Addresses</button>
    </PageHeader>
    <NiceTable :columns="columns" v-model:data="addresses"/>
  </div>
</template>