<script setup lang="ts">
import TableDisplayModal from "../modals/TableDisplayModal.vue";
const toast = useToast();
const { pushModal } = useModals();
type ServiceStatus = {
  name: string;
  status: string;
};

let $serviceStatus: ServiceStatus[] = [];
let $loading = false;

async function load() {
  $loading = true;
  const res = await apiCall("system.services.status", {});
  if (res.Error === null) {
    console.debug("services staus", res.Data);
    $serviceStatus = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

async function restart() {
  const res = await apiCall("system.power.restart", {});
  if (res.Error === null) toast.success("Restart Triggered");
  else console.debug("error", res);
}

async function shutdown() {
  const res = await apiCall("system.power.shutdown", {});
  if (res.Error === null) toast.success("Shutdown Triggered");
  else console.debug("error", res);
}

onMounted(load);
</script>

<template>
  <Page title="Dashboard">
    <template #header>
      <button @click="shutdown">Shutdown</button>
      <button @click="restart">Restart</button>
    </template>
    <template v-if="!$loading">
      <h2>Services</h2>
      <table>
        <thead>
          <tr>
            <th>Service</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(service, index) in $serviceStatus" :key="index">
            <td v-text="service.name" />
            <td v-text="service.status" />
          </tr>
        </tbody>
      </table>
    </template>
    <div v-else>Loading...</div>
  </Page>
</template>
<style scoped>
table {
  max-width: max-content;
}
</style>
