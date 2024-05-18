<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

let vm: any = $ref({});

async function create() {
  console.debug('value', vm);
  let res: any;
  res = await apiCall(`${subsystem }.${entity}.create`, vm);

  if (res.Error === null) {
    p.toast.success(`Created ${  editTypes[subsystem][entity].name}`);
    p.router.go(-1);
  } else {
    console.debug('error', res);
  }
}

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
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
