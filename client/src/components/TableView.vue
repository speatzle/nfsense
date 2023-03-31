<script setup lang="ts">

const props = defineModel<{
  title: string,
  loading: boolean,
  columns?: {
    heading: string,
    path: string,
    component?: Component,
  }[],
  data: Record<string, any>[],
  tableProps: any,
  selection?: number[],
}>();

let { title, columns, loadData, data, selection, tableProps } = $(props);

</script>

<template>
  <div>
    <PageHeader :title="title">
      <slot/>
    </PageHeader>
    <div v-if="loading" >Loading...</div>
    <NiceTable v-else :columns="columns" v-model:selection="selection" v-bind="tableProps" :data="data"/>
  </div>
</template>