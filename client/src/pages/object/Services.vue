<script setup lang="ts">
import { apiCall } from "../../api";

let services = $ref([]);
const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Type', path: 'type'},
  {heading: 'Value', path: 'value'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  let res = await apiCall("Object.GetServices", {});
  if (res.Error === null) {
    services = res.Data.Services;
    console.debug("services", services);
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
    <PageHeader title="services">
      <button @click="load">Load Services</button>
    </PageHeader>
    <NiceTable :columns="columns" v-model:data="services"/>
  </div>
</template>