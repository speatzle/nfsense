<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";
import UpsertModal from "../modals/UpsertModal.vue";

type TableAction = {
  caption: string;
  target: "single" | "any" | "pair" | "none";
  callback: (data: any) => unknown;
  enabled?: (data: any) => boolean;
};

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
  actions?: TableAction[];
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
  const res = await apiCall(`${$apiPath}.delete`, { id: $displayData[$selection[0]].name });
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
  if (await pushModal(UpsertModal, { subsystem: props.subsystem, entity: props.entity })) load();
}

async function editRow(index?: number) {
  const id = ($displayData[index ?? $selection[0]] ?? {}).name;
  if (await pushModal(UpsertModal, { subsystem: props.subsystem, entity: props.entity, id })) load();
}

function actionData(action: TableAction) {
  return action.target == "single" ? $displayData[$selection[0]] : $selection.map((i) => $displayData[i]);
}

function isActionEnabled(action: TableAction) {
  const isSelectionTarget = {
    single: () => $selection.length == 1,
    pair: () => $selection.length == 2,
    any: () => $selection.length >= 1,
    none: () => $selection.length == 0,
  }

  return isSelectionTarget[action.target]() && (!action.enabled || action.enabled(actionData(action)));
}

async function runAction(action: TableAction) {
  await action.callback(actionData(action));
  load();
}

onMounted(load);
</script>

<template>
  <Page :title="`${$entity.name}s`">
    <template #header>
      <button @click="load">Refresh</button>
      <button @click="createRow" :class="{accent: $selection.length != 1}">Create</button>
      <button :disabled="$selection.length != 1" :class="{accent: $selection.length == 1}" @click="() => editRow()">Edit</button>
      <button :disabled="$selection.length != 1" @click="deleteRow">Delete</button>
      <button :disabled="!isActionEnabled(action)" v-for="action of props.actions" v-text="action.caption" @click="runAction(action)"/>
    </template>
    <NiceTable
      v-model:selection="$selection"
      v-model:data="$displayData"
      :columns="$columns"
      :draggable="$entity.ordered"
      :sort="!$entity.ordered"
      :sort-self="!$entity.ordered"
      @dragged-row="moveRow"
      @row-action="(index) => editRow(index)"
    />
  </Page>
</template>
