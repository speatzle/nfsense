<script setup lang="ts">

import { editTypes } from "../../../../definitions";
import getPlugins from '../../../../plugins';
const p = getPlugins();

const props = $defineProps<{subsystem: string, entity: string, id: string}>();
const { subsystem, entity, id } = $(props);

let data = $ref({} as {});

async function update() {

}

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Edit ' + editTypes[subsystem][entity].name">
      <button @click="update">Update</button>
      <button @click="$router.go(-1)">Discard</button>
    </PageHeader>
    <NiceForm class="scroll cl-secondary" :sections="editTypes[subsystem][entity].sections" v-model="data"/>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
