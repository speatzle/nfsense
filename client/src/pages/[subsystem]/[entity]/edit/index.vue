<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';

const p = getPlugins();
const router = useRouter();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

const vm: any = $ref({});

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

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Create ' + editTypes[subsystem][entity].name"/>
    <NicerForm v-model="vm" class="scroll cl-secondary" :fields="editTypes[subsystem][entity].fields" :default="editTypes[subsystem][entity].default ?? undefined"/>
    <div class="actions">
      <div class="flex-grow"/>
      <button @click="create">Submit</button>
      <div class="space"/>
      <button @click="router.go(-1);">Discard</button>
      <div class="flex-grow"/>
    </div>
    <pre v-text="JSON.stringify(vm, null, 2)"/>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
