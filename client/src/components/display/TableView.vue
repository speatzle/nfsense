<script setup lang="ts">
const props = defineProps<{
  title: string;
  loading: boolean;
  columns?: {
    heading: string;
    path: string;
    component?: Component;
  }[];
  tableProps?: any;

  // Two-Way Bindings
  data: Record<string, any>[];
  selection?: number[];
}>();

const emit = defineEmits<{
  (event: "draggedRow", draggedRow: number, draggedOverRow: number): void;
  (event: "update:data", value: Record<string, any>[]): void;
  (event: "update:selection", value: number[]): void;
}>();

let $data = props.data;
syncModel(toRef(props, "data"), $$($data), (v) => emit("update:data", v));
let $selection = props.selection ?? [];
syncModel(toRef(props, "selection"), $$($selection), (v) => emit("update:selection", v));

async function draggedRow(draggedRow: number, draggedOverRow: number) {
  emit("draggedRow", draggedRow, draggedOverRow);
}
</script>

<template>
  <div>
    <PageHeader :title="props.title">
      <slot />
    </PageHeader>
    <div v-if="props.loading">Loading...</div>
    <NiceTable
      v-else
      v-model:selection="$selection"
      :columns="props.columns"
      v-bind="props.tableProps"
      v-model:data="$data"
      @dragged-row="draggedRow"
    />
  </div>
</template>
