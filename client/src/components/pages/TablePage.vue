<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";
import UpsertModal from "../modals/UpsertModal.vue";

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
}>();
const $subsystem = $(computed(() => subsystems[props.subsystem]));
const $entity = $(computed(() => ($subsystem.entities as Record<string, Entity>)[props.entity]));
const $columns = $(computed(() => $entity.columns ?? []));
const $apiPath = $(computed(() => `${props.subsystem}.${props.entity}`));

const toast = useToast();
const { pushModal } = useModals();

let $data = [] as any;
let $selection = [] as number[];

const $displayData = $(
  computed(() => ($entity.ordered ? $data : Object.entries($data).map(([_, v]) => v))),
);

async function load() {
  const res = await apiCall(`${$apiPath}.list`, {});
  if (res.Error === null) {
    $data = res.Data;
    $selection = [];
    console.debug(props.entity, $data);
  } else console.debug("error", res);
}

async function deleteRow() {
  const res = await apiCall(
    `${$apiPath}.delete`,
    $entity.ordered ? { index: $selection[0] } : { name: $displayData[$selection[0]].name },
  );
  if (res.Error === null) {
    console.debug(`Deleted ${$entity.name}`);
    toast.success(`Deleted ${$entity.name}`);
  } else console.debug("error", res);
  load();
}

async function moveRow(from: number, to: number) {
  if (!$entity.ordered) return;
  console.log("dragged", { from, to });
  const res = await apiCall(`${$apiPath}.move`, {
    index: from,
    to_index: to,
  });
  if (res.Error === null) {
    console.debug("moved rule");
    toast.success("Moved Rule");
  } else console.debug("error", res);

  load();
}

async function createRow() {
  if (await pushModal(UpsertModal, props)) load();
}

async function editRow() {
  const id = $entity.ordered ? $selection[0] : ($displayData[$selection[0]] ?? {}).name;
  if (await pushModal(UpsertModal, { ...props, id })) load();
}

onMounted(load);
</script>

<template>
  <Page :title="`${$entity.name}s`">
    <template #header>
      <button @click="load">Refresh</button>
      <button @click="createRow">Create</button>
      <button :disabled="$selection.length != 1" @click="editRow">Edit</button>
      <button :disabled="$selection.length != 1" @click="deleteRow">Delete</button>
    </template>
    <NiceTable
      v-model:selection="$selection"
      v-model:data="$displayData"
      :columns="$columns"
      :table-props="$entity.ordered ? { draggable: true } : { sort: true, sortSelf: true }"
      @dragged-row="moveRow"
    />
  </Page>
</template>
