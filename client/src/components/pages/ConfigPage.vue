<script setup lang="ts">
export type ChangeAction = "Create" | "Update" | "Delete";

export interface Change {
  path: string;
  action: ChangeAction;
  id: string;
  old_value?: any;
  new_value?: any;
}

export interface ChangeSet {
  user: string;
  timestamp: string;
  changes: Change[];
}

const p = usePlugins();

let $changelog = [] as ChangeSet[];
let $loading = false;

const columns = [
  { heading: "User", path: "user" },
  { heading: "Path", path: "path" },
  { heading: "Action", path: "action" },
  { heading: "ID", path: "id" },
];

const $displayData = $(
  computed(() => {
    const data = [];
    for (const changeSet of $changelog) {
      if (changeSet.changes) {
        for (const change of changeSet.changes) {
          data.push({
            user: changeSet.user,
            path: change.path,
            action: change.action,
            id: change.id,
          });
        }
      }
    }
    return data;
  }),
);

async function load() {
  $loading = true;
  const res = await apiCall("config.pending_changes.log", {});
  if (res.Error === null) {
    console.debug("changelog", res.Data);
    $changelog = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

async function apply() {
  const res = await apiCall("config.pending_changes.apply", {});
  if (res.Error === null) {
    console.debug("apply");
    p.toast.success("Applied Pending Config");
  } else console.debug("error", res);
  load();
}

async function discard() {
  const res = await apiCall("config.pending_changes.discard", {});
  if (res.Error === null) {
    console.debug("discard");
    p.toast.success("Discarded Pending Config");
  } else console.debug("error", res);
  load();
}

onMounted(load);
</script>

<template>
  <div>
    <PageHeader title="Pending Changes">
      <button @click="load">Refresh</button>
      <button @click="apply">Apply</button>
      <button @click="discard">Discard</button>
    </PageHeader>
    <TableView
      v-model:data="$displayData"
      :columns="columns"
      :loading="$loading"
      :table-props="{ sort: true, sortSelf: true }"
    />
  </div>
</template>
