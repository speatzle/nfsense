<script setup lang="ts">
import { editTypes } from '../../../../definitions';
import { apiCall } from '../../../../api';
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

async function create(value: any) {
  console.debug('value', value);
  let res: any;
  if (editTypes[subsystem][entity].idType == 'Number') {
    res = await apiCall(`${subsystem }.${entity}.create`, value);
  } else {
    // TODO find way to only have a name/id field in the form on create and not put it into the value
    let id = value.name;
    delete value.name;
    res = await apiCall(`${subsystem }.${entity}.create`, {id: id, thing: value});
  }

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
    <NiceForm class="scroll cl-secondary" :submit="create" :discard="() => $router.go(-1)" :sections="editTypes[subsystem][entity].sections"/>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
