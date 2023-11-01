<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string, id: string | number}>();
const { subsystem, entity, id } = $(props);

let initialValues = $ref({} as {});
let loading = $ref(true);

async function load(){
  loading = true;
  let res: any;
  if (editTypes[subsystem][entity].idType == 'Number') {
    res = await apiCall(`${subsystem}.${entity}.get`, {id: id as number - 0});
  } else {
    res = await apiCall(`${subsystem}.${entity}.get`, {id: id});
  }

  if (res.Error === null) {
    console.debug('update data', res.Data);
    initialValues = res.Data;
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function update(value: any) {
  console.debug('value', value);
  let res: any;

  if (editTypes[subsystem][entity].idType == 'Number') {
    res = await apiCall(`${subsystem}.${entity}.update`, {id: id as number - 0, thing: value});
  } else {
    // TODO dont have name in value at all, see create (index.vue)
    delete value.name;
    res = await apiCall(`${subsystem}.${entity}.update`, {id: id, thing: value});
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
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Update ' + editTypes[subsystem][entity].name">
    </PageHeader>
    <NiceForm v-if="!loading" class="scroll cl-secondary" :submit="update" :discard="() => $router.go(-1)" :sections="editTypes[subsystem][entity].sections" :initial-values="initialValues"/>
    <p v-else>Loading...</p>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>

