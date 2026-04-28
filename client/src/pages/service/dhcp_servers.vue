<script setup lang="ts">
import ArrayDisplay from "~/components/display/ArrayDisplay.vue";
const p = usePlugins();

let $servers = [] as any[];
const $loading = false;
const $selection = [] as number[];

const columns = [
  { heading: "Interface", path: "interface" },
  { heading: "Pool", path: "pool", component: markRaw(ArrayDisplay), componentProp: "data" },
  { heading: "Comment", path: "comment" },
];

async function load() {
  const res = await apiCall("service.dhcp_servers.list", {});
  if (res.Error === null) {
    $servers = res.Data;
    console.debug("rules", $servers);
  } else console.debug("error", res);
}

async function deleteRule() {
  const res = await apiCall("service.dhcp_servers.delete", { index: $selection[0] });
  if (res.Error === null) {
    console.debug("deleted server");
    p.toast.success("Deleted DHCP Server");
  } else console.debug("error", res);
  load();
}

onMounted(load);
</script>

<template>
  <div>
    <TableView
      v-model:selection="$selection"
      v-model:data="$servers"
      title="DHCP Servers"
      :columns="columns"
      :loading="$loading"
      :table-props="{ sort: true, sortSelf: true }"
    >
      <button @click="load">Refresh</button>
      <router-link class="button" to="/service/dhcp_servers/edit">Create</router-link>
      <router-link
        class="button"
        :class="{ disabled: $selection.length != 1 }"
        :to="'/service/dhcp_servers/edit/' + $selection[0]"
        >Edit</router-link
      >
      <button :disabled="$selection.length != 1" @click="deleteRule">Delete</button>
    </TableView>
  </div>
</template>
