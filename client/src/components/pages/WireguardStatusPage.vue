<script setup lang="ts">
let $status = "";
let $loading = false;

async function load() {
  $loading = true;
  const res = await apiCall("vpn.wireguard.status", {});
  if (res.Error === null) {
    console.debug("status", res.Data);
    $status = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

onMounted(load);
</script>

<template>
  <Page title="Wireguard Status">
    <template v-if="!$loading">
      <div v-for="(line, index) in $status.split('\n')" :key="index">
        <p>{{ line }}</p>
      </div>
    </template>
    <div v-else>Loading...</div>
  </Page>
</template>

<style scoped></style>
