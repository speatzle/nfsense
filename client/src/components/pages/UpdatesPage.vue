<script setup lang="ts">
const toast = useToast();

type UpdateStatus = {
  available_updates: any[];
  current_version: string;
};
type UpdateJob = any;

let $updateStatus: UpdateStatus = { available_updates: [], current_version: "" };
let $updateJobs: UpdateJob[] = [];
const $selection = [] as number[];
let $loading = false as boolean;

const columns = [
  { heading: "Version", path: "version" },
  { heading: "Newest", path: "newest" },
  { heading: "Available", path: "available" },
  { heading: "Installed", path: "installed" },
  { heading: "Obsolete", path: "obsolete" },
  { heading: "Protected", path: "protected" },
  { heading: "Incomplete", path: "incomplete" },
];

const $selected = $(computed(() => $updateStatus.available_updates[$selection[0]]));
const $selectedInstalled = $(computed(() => $selected && $selected?.installed));
const $selectedNonInstalled = $(computed(() => $selected && !$selected?.installed));

async function load() {
  $loading = true;
  const res = await apiCall("system.update.status", {});
  if (res.Error === null) {
    console.debug("update status", res.Data);
    $updateStatus = res.Data;
  } else {
    console.debug("error", res);
    $updateStatus = { available_updates: [], current_version: "" };
  }

  const res2 = await apiCall("system.update.jobs.list", {});
  if (res.Error === null) {
    console.debug("update jobs", res2.Data);
    $updateJobs = res2.Data;
  } else {
    console.debug("error", res2);
    $updateJobs = [];
  }
  $loading = false;
}

async function vacuum() {
  const res = await apiCall("system.update.vacuum", {});
  if (res.Error === null) {
    console.debug("vacuum triggered");
    toast.success("Vacuum Triggered");
  } else console.debug("error", res);
}

async function acquire() {
  const res = await apiCall("system.update.acquire", {
    version: $updateStatus.available_updates[$selection[0]].version,
  });
  if (res.Error === null) {
    console.debug("acquire triggered");
    toast.success("acquire Triggered");
  } else console.debug("error", res);
}

async function install() {
  const res = await apiCall("system.update.install", {
    version: $updateStatus.available_updates[$selection[0]].version,
  });
  if (res.Error === null) {
    console.debug("install triggered");
    toast.success("Installation Triggered");
    // TODO Add progress bar
  } else console.debug("error", res);
}

async function setDefaultBoot() {
  const res = await apiCall("system.update.set_default_boot", {
    version: $updateStatus.available_updates[$selection[0]].version,
  });
  if (res.Error === null) {
    console.debug("setting default boot");
    toast.success("Default Version Set");
  } else console.debug("error", res);
}

async function details() {
  // TODO show changelog
  alert(`Content: ${$updateStatus.available_updates[$selection[0]].content}`);
}

onMounted(load);
</script>

<template>
  <Page title="Updates">
    <template #header>
      <button @click="load">Refresh</button>
      <button @click="vacuum">Vacuum</button>
      <button :disabled="!$selectedNonInstalled" @click="acquire">Acquire</button>
      <button :disabled="!$selectedNonInstalled" @click="install">Install</button>
      <button :disabled="!$selectedInstalled" @click="setDefaultBoot">Set as Default Boot</button>
      <button :disabled="$selection.length != 1" @click="details">Details</button>
    </template>
    <template v-if="!$loading">
      Current Version: {{ $updateStatus.current_version }}
      <div v-if="$updateJobs.length">
        <div v-for="job in $updateJobs">
          Job ID: {{ job.id }} Type: {{ job.type }} Progress: {{ job.progress }}
        </div>
      </div>
      <NiceTable
        v-model:selection="$selection"
        v-model:data="$updateStatus.available_updates"
        title="Updates"
        :columns="columns"
        :loading="$loading"
      />
    </template>
  </Page>
</template>
