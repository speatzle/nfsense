<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";
const p = usePlugins();

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
  id: string | number;
}>();
const $subsystem = $(computed(() => subsystems[props.subsystem]));
const $entity = $(computed(() => ($subsystem.entities as Record<string, Entity>)[props.entity]));

let $vm = {} as any; // TODO: Add proper type
let $loading = true;

async function load() {
  $loading = true;
  let res: any;
  if ($entity.idType == "Number")
    res = await apiCall(`${props.subsystem}.${props.entity}.get`, {
      index: (props.id as number) - 0,
    });
  else res = await apiCall(`${props.subsystem}.${props.entity}.get`, { name: props.id });

  if (res.Error === null) {
    console.debug("update data", res.Data);
    $vm = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

async function update() {
  console.debug("value", $vm);
  let res: any;

  if ($entity.idType === "Number")
    res = await apiCall(`${props.subsystem}.${props.entity}.update`, {
      index: Number(props.id),
      thing: $vm,
    });
  else if (props.id === $vm.name || confirm("Do you want to change the name & all references?"))
    res = await apiCall(`${props.subsystem}.${props.entity}.update`, {
      name: props.id,
      thing: $vm,
    });

  if (res.Error === null) {
    p.toast.success(`Updated ${$entity.name}`);
    p.router.go(-1);
  } else console.debug("error", res);
}

onMounted(async () => {
  if ($entity) load();
});
</script>
<template>
  <p v-if="$loading">Loading...</p>
  <div v-else-if="$entity">
    <PageHeader :title="'Update ' + $entity.name"> </PageHeader>
    <NicerForm v-model="$vm" class="scroll cl-secondary" :fields="$entity.fields" />
    <div class="actions">
      <div class="flex-grow" />
      <button @click="update">Submit</button>
      <div class="space" />
      <button @click="$router.go(-1)">Discard</button>
      <div class="flex-grow" />
    </div>
    <pre v-text="JSON.stringify($vm, null, 2)" />
  </div>
  <div v-else>
    <PageHeader title="Error" />
    No editType for this Entity
  </div>
</template>
