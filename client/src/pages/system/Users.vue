<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let users = $ref([]);
let loading = $ref(false);
let selection = $ref([] as number[]);

const columns = [
  {heading: 'Name', path: 'name'},
  {heading: 'Comment', path: 'comment'},
];

async function load(){
  loading = true;
  let res = await apiCall('system.get_users', {});
  if (res.Error === null) {
    users = res.Data;
    console.debug('users', users);
  } else {
    console.debug('error', res);
  }
  loading = false;
}

const displayData = $computed(() => {
  let data: any;
  data = [];
  for (const name in users) {
    data.push({
      name,
      comment: users[name].comment,
    });
  }
  return data;
});


async function deleteUser(){
  let res = await apiCall('system.delete_user', {name: displayData[selection[0]].name});
  if (res.Error === null) {
    console.debug('deleted user');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editUser() {
  p.router.push(`/system/users/edit/${  displayData[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView title="Users" :columns="columns" :loading="loading" v-model:selection="selection" v-model:data="displayData" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/system/users/edit">Create</router-link>
    <button @click="editUser" :disabled="selection.length != 1">Edit</button>
    <button @click="deleteUser" :disabled="selection.length != 1">Delete</button>
  </TableView>
</template>