<script setup lang="ts">
import { apiCall } from "../api";

async function doShit(){
  apiCall("Firewall.GetForwardRules", {});
}

let name = $ref("");
let comment = $ref("");
let counter = $ref(false);
let options = $ref([{name: 'Accept'}, {name: 'Drop'}, {name: 'Continue'}]);

</script>

<template>
  <div style="overflow-y: auto;">
    <PageHeader title="Dashboard">
      <button @click="doShit">Example Buttons</button>
    </PageHeader>
    <form @submit="$event => $event.preventDefault()" class="cl-secondary">
      <h3>Create Rule</h3>
      <label for="name" v-text="'Name'"/>
      <input name="name" v-model="name"/>
      <label for="counter" v-text="'Counter'"/>
      <input name="counter" type="checkbox" v-model="counter"/>
      <label for="comment" v-text="'Comment'"/>
      <textarea name="comment" v-model="comment"></textarea>
      <label for="verdict" v-text="'Verdict'"/>
      <pillbar :options="options" name="verdict" ></pillbar>
      <button>Submit</button>
    </form>
    <Multiselect/>
    asd
  </div>
</template>

<style scoped>

</style>