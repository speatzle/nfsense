<script setup lang="ts">
import { editTypes } from "../../../../definitions";

const props = $defineProps<{subsystem: string, entity: string}>();
const { subsystem, entity } = $(props);

async function create() {
  
}

</script>
<template>
  <div v-if="editTypes[subsystem][entity]">
    <PageHeader :title="'Edit ' + editTypes[subsystem][entity].title">
      <button @click="create">Create</button>
      <button @click="$router.go(-1)">Discard</button>
    </PageHeader>
    <NiceForm class="scroll cl-secondary" :title="editTypes[subsystem][entity].title" :fields="editTypes[subsystem][entity].fields"/>
  </div>
  <div v-else>
    <PageHeader title="Error"/>
    No editType for this Entity
  </div>
</template>
