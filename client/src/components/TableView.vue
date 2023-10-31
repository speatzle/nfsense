<script setup lang="ts">

const props = defineModels<{
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

let { title, loading, columns, data, selection, tableProps } = $(props);

const emit = defineEmits<{
  (event: 'draggedRow', draggedRow: number, draggedOverRow: number): void,
}>();

async function draggedRow(draggedRow: number, draggedOverRow: number) {
  emit("draggedRow", draggedRow, draggedOverRow);
}

</script>

<template>
  <div>
    <PageHeader :title="title">
      <slot/>
    </PageHeader>
    <div v-if="loading" >Loading...</div>
    <NiceTable v-else :columns="columns" v-model:selection="selection" @draggedRow="draggedRow" v-bind="tableProps" :data="data"/>
  </div>
</template>