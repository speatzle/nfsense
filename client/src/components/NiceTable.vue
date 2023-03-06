<script setup lang="ts">
const props = defineModel<{
  columns?: Record<string, string>,
  data?: Record<string, any>[],
  sortSelf?: boolean,
  sortBy?: string,
  sortDesc?: boolean,
}>();
let { columns, data, sortSelf, sortBy, sortDesc } = $(props);

const displayData = $computed(() => (sortSelf && sortBy !== '')
  ? data?.sort((a, b) => {
    let result;
    if (a[sortBy ?? ''] > b[sortBy ?? '']) result = 1;
    else if (a[sortBy ?? ''] === b[sortBy ?? '']) result = 0;
    else result = -1;

    if (sortDesc) return -result;
    return result;
  })
  : data);

function toggleSorting(columnName: string) {
  if (columnName === sortBy) sortDesc = !sortDesc;
  else {
    sortDesc = false;
    sortBy = columnName;
  }
}
</script>

<template>
  <table>
    <thead>
      <tr>
        <th v-for="[name, heading] in Object.entries(columns ?? {})" :key="name" @click="toggleSorting(name)">
          <div class="flex-row">
            {{ heading }}
            <i-mdi-arrow-down v-if="name === sortBy && sortDesc"/>
            <i-mdi-arrow-up v-else-if="name === sortBy"/>
          </div>
        </th>
      </tr>
    </thead>
    <tbody>
      <!-- eslint-disable-next-line vue/require-v-for-key -->
      <tr v-for="row of displayData">
        <td v-for="cell in row" :key="cell" v-text="cell"/>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
</style>