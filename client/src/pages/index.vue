<script setup lang="ts">
import { apiCall } from "../api";
import Multiselect from 'vue-multiselect';

async function doShit(){
  apiCall("Firewall.GetForwardRules", {});
}
let value = $ref("");
let options = [{ name: 'Vue.js', code: 'vu' },
  { name: 'Javascript', code: 'js' },
  { name: 'Open Source', code: 'os' }];

</script>

<template>
  <div>
    <PageHeader title="Dashboard">
      <button @click="doShit">Example Buttons</button>
    </PageHeader>
    <multiselect v-model="value" placeholder="Search" label="name" track-by="code" :options="options" :multiple="true"></multiselect>
  </div>
</template>

<style src="vue-multiselect/dist/vue-multiselect.css"></style>