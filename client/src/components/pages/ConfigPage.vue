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

const fileInput = useTemplateRef<HTMLInputElement>("fileInput");

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
  const res = await apiCall("config.pending.log", {});
  if (res.Error === null) {
    console.debug("changelog", res.Data);
    $changelog = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

async function apply() {
  const res = await apiCall("config.pending.apply", {});
  if (res.Error === null) {
    console.debug("apply");
    p.toast.success("Applied Pending Config");
  } else console.debug("error", res);
  load();
}

async function discard() {
  const res = await apiCall("config.pending.discard", {});
  if (res.Error === null) {
    console.debug("discard");
    p.toast.success("Discarded Pending Config");
  } else console.debug("error", res);
  load();
}

async function exportConfig() {
  const res = await apiCall("config.pending.get", {});
  if (res.Error === null) {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(res.Data, null, 2));
    const downloadAnchor = document.createElement("a");
    downloadAnchor.setAttribute("href", dataStr);
    downloadAnchor.setAttribute("download", "config.json");
    document.body.appendChild(downloadAnchor);
    downloadAnchor.click();
    downloadAnchor.remove();
    p.toast.success("Config exported successfully");
  } else console.debug("error", res);
}

function triggerImport() {
  fileInput.value?.click();
}

async function importConfig(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = async (e) => {
    try {
      const config = JSON.parse(e.target?.result as string);
      const res = await apiCall("config.pending.set", { config });
      if (res.Error === null) {
        p.toast.success("Config imported successfully");
        load();
      } else console.debug("error", res);
    } catch (err: any) {
      p.toast.error("Failed to parse JSON file: " + err.message);
    }
  };
  reader.readAsText(file);
  target.value = "";
}

onMounted(load);
</script>

<template>
  <div>
    <PageHeader title="Pending Changes">
      <button @click="load">Refresh</button>
      <button @click="triggerImport">Import</button>
      <button @click="exportConfig">Export</button>
      <input ref="fileInput" type="file" accept=".json" style="display: none" @change="importConfig" />
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
