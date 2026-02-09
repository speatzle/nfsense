<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

type UpdateStatus = any

let updateStatus: UpdateStatus = $ref(null);
const selection = $ref([] as any[]);
let loading = $ref(false);

const columns = [
  { heading: 'Version', path: 'version' },
  { heading: 'Newest', path: 'newest' },
  { heading: 'Available', path: 'available' },
  { heading: 'Installed', path: 'installed' },
  { heading: 'Obsolete', path: 'obsolete' },
  { heading: 'Protected', path: 'protected' },
  { heading: 'Incomplete', path: 'incomplete' },
];

async function load(){
  loading = true;
  const res = await apiCall('system.update.status', {});
  if (res.Error === null) {
    console.debug('update status', res.Data);
    updateStatus = res.Data;
  } else {
    console.debug('error', res);
    updateStatus = null;
  }
  loading = false;
}

async function vacuum(){
  const res = await apiCall('system.update.vacuum', {});
  if (res.Error === null) {
    console.debug('vacuum triggered');
    p.toast.success('Vacuum Triggered');
  } else {
    console.debug('error', res);
  }
}

async function install(){
  const res = await apiCall('system.update.install', { version: updateStatus.available_updates[selection[0]].version } );
  if (res.Error === null) {
    console.debug('install triggered');
    p.toast.success('Installation Triggered');
    // TODO Add progress bar
  } else {
    console.debug('error', res);
  }
}

async function setDefaultBoot(){
  const res = await apiCall('system.update.set_default_boot', { version: updateStatus.available_updates[selection[0]].version });
  if (res.Error === null) {
    console.debug('setting default boot');
    p.toast.success('Default Version Set');
  } else {
    console.debug('error', res);
  }
}

async function details() {
  // TODO show changelog
  alert(`Content: ${updateStatus.available_updates[selection[0]].content}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <div>
    <template v-if="!loading">
      <div v-if="updateStatus">
        Current Version: {{ updateStatus.current_version }}
        <div v-if="updateStatus.job">
          Job ID: {{ updateStatus.job.id }}
          Type: {{ updateStatus.job.type }}
          Progress: {{ updateStatus.job.progress }}
        </div>
      </div>
    </template>
    <TableView v-model:selection="selection" v-model:data="updateStatus.available_updates" title="Updates" :columns="columns" :loading="loading">
      <button @click="load">Refresh</button>
      <button @click="vacuum">Vacuum</button>
      <button :disabled="!(selection.length == 1 && !updateStatus.available_updates[selection[0]].installed)" @click="install">Install</button>
      <button :disabled="!(selection.length == 1 && updateStatus.available_updates[selection[0]].installed)" @click="setDefaultBoot">Set as Default Boot</button>
      <button :disabled="selection.length != 1" @click="details">Details</button>
    </TableView>
  </div>
</template>
