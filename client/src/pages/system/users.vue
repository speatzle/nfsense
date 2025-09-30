<script setup lang="ts">
import { apiCall } from '../../api';
import getPlugins from '../../plugins';
const p = getPlugins();

let users = $ref([] as any[]); // TODO: Add proper type
let loading = $ref(false);
const selection = $ref([] as number[]);

const columns = [
  { heading: 'Name', path: 'name' },
  { heading: 'Comment', path: 'comment' },
];

async function load(){
  loading = true;
  const res = await apiCall('system.users.list', {});
  if (res.Error === null) {
    users = res.Data;
    console.debug('users', users);
  } else {
    console.debug('error', res);
  }
  loading = false;
}

async function deleteUser(){
  const res = await apiCall('system.users.delete', { name: users[selection[0]].name });
  if (res.Error === null) {
    console.debug('deleted user');
  } else {
    console.debug('error', res);
  }
  load();
}

async function editUser() {
  p.router.push(`/system/users/edit/${  users[selection[0]].name}`);
}

onMounted(async() => {
  load();
});

</script>

<template>
  <TableView v-model:selection="selection" v-model:data="users" title="Users" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
    <router-link class="button" to="/system/users/edit">Create</router-link>
    <button :disabled="selection.length != 1" @click="editUser">Edit</button>
    <button :disabled="selection.length != 1" @click="deleteUser">Delete</button>
  </TableView>
</template>
