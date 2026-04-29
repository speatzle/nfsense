<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";

const p = usePlugins();

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
}>();
const $subsystem = $(computed(() => subsystems[props.subsystem]));
const $entity = $(computed(() => ($subsystem.entities as Record<string, Entity>)[props.entity]));

const $vm: any = {};

async function create() {
  console.debug("value", $vm);
  const res = await apiCall(`${props.subsystem}.${props.entity}.create`, $vm);

  if (res.Error === null) {
    p.toast.success(`Created ${$entity.name}`);
    p.router.go(-1);
  } else console.debug("error", res);
}
</script>
<template>
  <div v-if="$entity">
    <PageHeader :title="'Create ' + $entity.name" />
    <NicerForm
      v-model="$vm"
      class="scroll cl-secondary"
      :fields="$entity.fields"
      :default="$entity.default ?? undefined"
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
