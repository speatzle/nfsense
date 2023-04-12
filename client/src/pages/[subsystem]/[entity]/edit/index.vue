<script setup lang="ts">
import { editTypes } from "../../../../definitions";
import { apiCall } from "../../../../api";
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

async function create(value: any) {
  console.debug("value", value);
  let res = await apiCall(editTypes[subsystem].name +".Create"+ editTypes[subsystem][entity].name, value);
  if (res.Error === null) {
    p.toast.success("Created " + editTypes[subsystem][entity].name);
    p.router.go(-1);
  } else {
    console.debug("error", res);
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
