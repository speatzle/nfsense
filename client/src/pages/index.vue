<script setup lang="ts">
import { apiCall } from "../api";

async function doShit(){
  apiCall("Firewall.GetForwardRules", {});
}

</script>

<template>
  <div>
    <PageHeader title="Dashboard">
      <button @click="doShit">Example Buttons</button>
    </PageHeader>

    This is the main page, currently written in markdown because that's *pog*.
  </div>
</template>