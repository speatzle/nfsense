<script setup lang="ts">
import ArrayDisplay from "~/components/display/ArrayDisplay.vue";
import UpsertModal from "../modals/UpsertModal.vue";
const { pushModal } = useModals();

let $interfaces = {} as any; // TODO: Add proper type
let $loading = false;
let $selection = [] as number[];

const columns = [
  { heading: "Name", path: "name" },
  { heading: "Listen Port", path: "listen_port" },
  { heading: "Peers", path: "peers", component: markRaw(ArrayDisplay), componentProp: "data" },
  { heading: "Comment", path: "comment" },
];

const $displayData = $(
  computed(() => {
    const data = [];
    for (const index in $interfaces)
      data.push({
        name: $interfaces[index].name,
        listen_port: $interfaces[index].listen_port,
        peers: $interfaces[index].peers,
        comment: $interfaces[index].comment,
      });
    return data;
  }),
);

async function load() {
  $loading = true;
  const res = await apiCall("vpn.wireguard.interfaces.list", {});
  if (res.Error === null) {
    console.debug("interfaces", res.Data);
    $interfaces = res.Data;
    $selection = [];
  } else console.debug("error", res);
  $loading = false;
}

async function generateKeys() {
  const res = await apiCall("vpn.wireguard.keypair.generate", {});
  if (res.Error === null) {
    console.debug("keypair", res.Data);
    alert(JSON.stringify(res.Data));
  } else console.debug("error", res);
}

async function deleteInterface() {
  const res = await apiCall("vpn.wireguard.interfaces.delete", {
    name: $displayData[$selection[0]].name,
  });
  if (res.Error === null) console.debug("deleted interface");
  else console.debug("error", res);
  load();
}

async function editInterface() {
  const id = $displayData[$selection[0]].name;
  if (await pushModal(UpsertModal, { subsystem: "vpn", entity: "wireguard.interfaces", id }))
    load();
}
async function createInterface() {
  if (await pushModal(UpsertModal, { subsystem: "vpn", entity: "wireguard.interfaces" })) load();
}

onMounted(load);
</script>

<template>
  <Page title="Wireguard Interfaces">
    <template #header>
      <button @click="load">Refresh</button>
      <button @click="generateKeys">Generate Keys</button>
      <button @click="createInterface">Create</button>
      <button :disabled="$selection.length != 1" @click="editInterface">Edit</button>
      <button :disabled="$selection.length != 1" @click="deleteInterface">Delete</button>
    </template>
    <NiceTable
      v-model:selection="$selection"
      v-model:data="$displayData"
      :columns="columns"
      :loading="$loading"
      :table-props="{ sort: true, sortSelf: true }"
    />
  </Page>
</template>
