<script setup lang="ts">
import { editTypes } from "../../../../definitions";
import { apiCall } from "../../../../api";
import { useToast } from 'vue-toast-notification';

const $toast = useToast();

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

let data = $ref({});

async function create(value: any) {
  console.debug("value", value);
  let res = await apiCall(editTypes[subsystem].name +".Create"+ editTypes[subsystem][entity].name, value);
  if (res.Error === null) {
    $toast.success("Created " + editTypes[subsystem][entity].name);
    $router.go(-1)
  } else {
    console.debug("error", res);
  }
}

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Create ' + editTypes[subsystem][entity].name">
    </PageHeader>
    <NiceForm class="scroll cl-secondary" :submit="create" :discard="() => $router.go(-1)" :sections="editTypes[subsystem][entity].sections" v-model="data"/>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
