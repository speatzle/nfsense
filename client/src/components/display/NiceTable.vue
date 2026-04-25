<script setup lang="ts">
import { useKeyModifier } from "@vueuse/core";
import type { Component, WatchOptions } from "vue";

const $shiftState = $(useKeyModifier("Shift"));
const $ctrlState = $(useKeyModifier("Control"));

const props = withDefaults(
  defineProps<{
    // Two-Way Bindings
    data?: Record<string, any>[];
    selection?: number[];
    sort?: boolean;
    sortBy?: string;
    sortDesc?: boolean;

    // One-Way Bindings
    sortSelf?: boolean;
    draggable?: boolean;
    columns?: {
      heading: string;
      path: string;
      component: Component;
      props: any;
    }[];
  }>(),
  {
    data: () => [],
    selection: () => [],
    sort: false,
    sortBy: "",
    sortDesc: false,
    sortSelf: false,
    draggable: false,
    columns: () => [],
  },
);

const emit = defineEmits<{
  (event: "update:data", value: Record<string, any>[]): void;
  (event: "update:selection", value: number[]): void;
  (event: "update:sort", value: boolean): void;
  (event: "update:sortBy", value: string): void;
  (event: "update:sortDesc", value: boolean): void;
  (event: "rowAction", index: number): void;
  (event: "draggedRow", draggedRow: number, draggedOverRow: number): void;
}>();

// Hook up two-way bindings
let $data = props.data;
syncModel(toRef(props, "data"), $$($data), (v) => emit("update:data", v), true);
let $sort = props.sort;
syncModel(toRef(props, "sort"), $$($sort), (v) => emit("update:sort", v));
let $sortBy = props.sortBy;
syncModel(toRef(props, "sortBy"), $$($sortBy), (v) => emit("update:sortBy", v));
let $sortDesc = props.sortDesc;
syncModel(toRef(props, "sortDesc"), $$($sortDesc), (v) => emit("update:sortDesc", v));
let $selection = props.selection;
syncModel(toRef(props, "selection"), $$($selection), (v) => emit("update:selection", v), true);

const $displayData = $(
  computed(() =>
    props.sortSelf && $sortBy !== ""
      ? $data?.sort((a, b) => {
          // TODO Determine whether sorting a copy is necessary
          $selection = [];
          let result;
          if (a[$sortBy ?? ""] > b[$sortBy ?? ""]) result = 1;
          else if (a[$sortBy ?? ""] === b[$sortBy ?? ""]) result = 0;
          else result = -1;

          if ($sortDesc) return -result;
          return result;
        })
      : $data,
  ),
);

function toggleSorting(columnName: string) {
  if (!$sort) return;
  if (columnName === $sortBy) $sortDesc = !$sortDesc;
  else [$sortDesc, $sortBy] = [false, columnName];
}

function toggleRowSelection(index: number) {
  if (!$selection || !$selection.length) $selection = [index];
  else if ($shiftState) {
    // Selection becomes a range including the highest, lowest and clicked row
    const points = [Math.max(...$selection), Math.min(...$selection), index];
    const [max, min] = [Math.max(...points), Math.min(...points)];
    $selection = Array.from({ length: max - min + 1 }, (_, i) => i + min);
  } else if ($ctrlState)
    // Toggle the presence of the row in the selection
    $selection = $selection.includes(index)
      ? $selection.filter((i) => i !== index)
      : [...$selection, index];
  else $selection = $selection.includes(index) ? [] : [index]; // Toggle between selection of none and this row
}

let $draggedRow = -1;
let $draggedOverRow = -1;
function dragDropRow() {
  if ($data) {
    const row = $data[$draggedRow];
    $data.splice($draggedRow, 1);
    $data.splice($draggedOverRow, 0, row);
    $data = [...$data];
    // Don't emit if we are at the same spot
    if ($draggedRow !== $draggedOverRow) emit("draggedRow", $draggedRow, $draggedOverRow);
  }

  // Reset Drag & Remove Selection
  $draggedRow = 0;
  $draggedOverRow = 0;
  $selection = [];
}
</script>

<template>
  <table>
    <thead>
      <tr>
        <th
          v-for="{ heading, path } of props.columns"
          :key="path"
          @click="() => toggleSorting(path)"
        >
          <div class="flex-row">
            {{ heading }}
            <template v-if="$sort">
              <i-mdi-arrow-down v-if="path === $sortBy && $sortDesc" />
              <i-mdi-arrow-up v-else-if="path === $sortBy" />
            </template>
          </div>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(row, index) in $displayData"
        :key="index"
        :draggable="draggable"
        :class="{
          selected: ($selection ?? []).includes(index),
          'dragged-over-before': index === $draggedOverRow && $draggedOverRow < $draggedRow,
          'dragged-over-after': index === $draggedOverRow && $draggedOverRow > $draggedRow,
        }"
        @click="() => toggleRowSelection(index)"
        @dblclick="() => emit('rowAction', index)"
        @dragstart="() => ($draggedRow = index)"
        @dragenter="() => ($draggedOverRow = index)"
        @dragend="() => dragDropRow()"
      >
        <td v-for="col of columns" :key="col.path">
          <component
            :is="col.component"
            v-if="col.component"
            :data="atPath(row, col.path)"
            v-bind="col.props"
          />
          <template v-else>{{ atPath(row, col.path) }}</template>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.dragged-over-before {
  border-top: 0.25rem solid var(--cl-fg);
}
.dragged-over-after {
  border-bottom: 0.25rem solid var(--cl-fg);
}

tr {
  cursor: grab;
}
</style>
