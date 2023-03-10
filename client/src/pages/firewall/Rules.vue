<script setup lang="ts">
import { apiCall } from "../../api";

let rules = $ref([]);

async function loadRules(){
  let res = await apiCall("Firewall.GetForwardRules", {});
  if (res.Error === null) {
    rules = res.Data.ForwardRules;
    console.debug("rules", rules);
  } else {
    console.debug("error", res);
  }
}

onMounted(async() => {
  loadRules();
});

</script>

<template>
  <div>
    <PageHeader title="Forward Rules">
      <button @click="loadRules">Load Rules</button>
    </PageHeader>
    <NiceTable :columns="{name: 'Name', verdict: 'Verdict'}" :sort-self="false" v-model:data="rules"/>
  </div>
</template>