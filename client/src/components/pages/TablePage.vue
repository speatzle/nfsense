<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
}>();
const $subsystem = $(computed(() => subsystems[props.subsystem]));
const $entity = $(computed(() => ($subsystem.entities as Record<string, Entity>)[props.entity]));
const $columns = $(computed(() => $entity.columns ?? []));
const $apiPath = $(computed(() => `${props.subsystem}.${props.entity}`));
const $editUrl = $(computed(() => `/${props.subsystem}/${props.entity}/edit`));

const p = usePlugins();

let $data = [] as any;
const $loading = false;
const $selection = [] as number[];

const $displayData = $(
  computed(() => ($entity.ordered ? $data : Object.entries($data).map(([_, v]) => v))),
);

async function load() {
  const res = await apiCall(`${$apiPath}.list`, {});
  if (res.Error === null) {
    $data = res.Data;
    console.debug(props.entity, $data);
  } else console.debug("error", res);
}

async function deleteEntry() {
  const res = await apiCall(
    `${$apiPath}.delete`,
    $entity.ordered ? { index: $selection[0] } : { name: $displayData[$selection[0]].name },
  );
  if (res.Error === null) {
    console.debug(`Deleted ${$entity.name}`);
    p.toast.success(`Deleted ${$entity.name}`);
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
    p.toast.success("Moved Rule");
  } else console.debug("error", res);

  load();
}

onMounted(load);
</script>

<template>
  <div>
    <TableView
      v-model:selection="$selection"
      v-model:data="$displayData"
      :title="`${$entity.name}s`"
      :columns="$columns"
      :loading="$loading"
      :table-props="$entity.ordered ? { draggable: true } : { sort: true, sortSelf: true }"
      @dragged-row="moveRow"
    >
      <button @click="load">Refresh</button>
      <router-link class="button" :to="$editUrl">Create</router-link>
      <router-link
        class="button"
        :class="{ disabled: $selection.length != 1 }"
        :to="`${$editUrl}/${$entity.ordered ? $selection[0] : ($displayData[$selection[0]] ?? {}).name}`"
        >Edit</router-link
      >
      <button :disabled="$selection.length != 1" @click="deleteEntry">Delete</button>
    </TableView>
  </div>
</template>
