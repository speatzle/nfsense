<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';
import { isNullish } from '../../../../util';

const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

let vm: any = $ref({});
let loading = $ref(true);

// Load default
async function load(){
  loading = true;
  if (!isNullish(editTypes[subsystem][entity].default)) {
    console.debug('loading form default', editTypes[subsystem][entity].default);
    vm = editTypes[subsystem][entity].default;
  } else {
    console.debug('no form default found');
  }
  loading = false;
}

async function create() {
  console.debug('value', vm);
  const res = await apiCall(`${subsystem }.${entity}.create`, vm);

  if (res.Error === null) {
    p.toast.success(`Created ${  editTypes[subsystem][entity].name}`);
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
    <PageHeader :title="'Create ' + editTypes[subsystem][entity].name">
    </PageHeader>
    <NicerForm v-model="vm" class="scroll cl-secondary" :fields="editTypes[subsystem][entity].fields"/>
    <div class="actions">
      <div class="flex-grow"/>
      <button @click="create">Submit</button>
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
