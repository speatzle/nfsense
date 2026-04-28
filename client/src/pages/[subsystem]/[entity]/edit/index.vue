<script setup lang="ts">
import { editTypes } from "~/definitions";

const p = usePlugins();

const props = defineProps<{ subsystem: string; entity: string }>();

const $vm: any = {};

async function create() {
  console.debug("value", $vm);
  const res = await apiCall(`${props.subsystem}.${props.entity}.create`, $vm);

  if (res.Error === null) {
    p.toast.success(`Created ${editTypes[props.subsystem][props.entity].name}`);
    p.router.go(-1);
  } else console.debug("error", res);
}
</script>
<template>
  <div v-if="editTypes[props.subsystem][props.entity]">
    <PageHeader :title="'Create ' + editTypes[props.subsystem][props.entity].name" />
    <NicerForm
      v-model="$vm"
      class="scroll cl-secondary"
      :fields="editTypes[props.subsystem][props.entity].fields"
      :default="editTypes[props.subsystem][props.entity].default ?? undefined"
    />
    <div class="actions">
      <div class="flex-grow" />
      <button @click="create">Submit</button>
      <div class="space" />
      <button @click="p.router.go(-1)">Discard</button>
      <div class="flex-grow" />
    </div>
    <pre v-text="JSON.stringify($vm, null, 2)" />
  </div>
  <div v-else>
    <PageHeader title="Error" />
    No editType for this Entity
  </div>
</template>
