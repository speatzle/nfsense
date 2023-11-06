<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string, id: string | number}>();
const { subsystem, entity, id } = $(props);

let vm = $ref({});
let loading = $ref(true);

async function load(){
  loading = true;
  let res: any;
  if (editTypes[subsystem][entity].idType == 'Number') {
    res = await apiCall(`${subsystem}.${entity}.get`, {index: id as number - 0});
  } else {
    res = await apiCall(`${subsystem}.${entity}.get`, {name: id});
  }

  if (res.Error === null) {
    console.debug('update data', res.Data);
    vm = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function update() {
  console.debug('value', vm);
  let res: any;

  if (editTypes[subsystem][entity].idType == 'Number') {
    res = await apiCall(`${subsystem}.${entity}.update`, {index: id as number - 0, thing: vm});
  } else {
    res = await apiCall(`${subsystem}.${entity}.update`, {name: id, thing: vm});
  }

  if (res.Error === null) {
    p.toast.success(`Updated ${  editTypes[subsystem][entity].name}`);
    p.router.go(-1);
  } else {
    console.debug('error', res);
  }
}

onMounted(async() => {
  if (editTypes[subsystem][entity]) {
    load();
  }
});

</script>
<template>
  <p v-if="loading">Loading...</p>
  <div v-else-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Update ' + editTypes[subsystem][entity].name">
    </PageHeader>
    <NicerForm class="scroll cl-secondary" :fields="editTypes[subsystem][entity].fields" v-model="vm"/>
    <div class="actions">
      <div class="flex-grow"/>
      <button @click="update">Submit</button>
      <div class="space"/>
      <button @click="$router.go(-1)">Discard</button>
      <div class="flex-grow"/>
    </div>
    <p>{{ vm }}</p>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
