<script setup lang="ts">
import { useKeyModifier } from '@vueuse/core';

const shiftState = $(useKeyModifier('Shift'));
const ctrlState = $(useKeyModifier('Control'));

const props = defineModels<{
  columns?: {
    heading: string,
    path: string,
    component?: string,
  }[], // -
  data?: Record<string, any>[],
  sort?: boolean,
  sortSelf?: boolean, // -
  sortBy?: string,
  sortDesc?: boolean,
  selection?: number[],
  draggable?: boolean, // -
}>();
let { columns, data, sort, sortSelf, sortBy, sortDesc, selection, draggable } = $(props);

const emit = defineEmits<{
  (event: 'rowAction', index: number): void,
  (event: 'selectionChanged'): void
  (event: 'draggedRow', draggedRow: number, draggedOverRow: number): void,
}>();

const componentProp: Record<string, string> = {
  "EnumDisplay": "data",
};


const displayData = $computed(() => (sortSelf && sortBy !== '')
  ? data?.sort((a, b) => {
    selection = [];
    let result;
    if (a[sortBy ?? ''] > b[sortBy ?? '']) result = 1;
    else if (a[sortBy ?? ''] === b[sortBy ?? '']) result = 0;
    else result = -1;

    if (sortDesc) return -result;
    return result;
  })
  : data);

function toggleSorting(columnName: string) {
  if (!sort) return;
  if (columnName === sortBy) sortDesc = !sortDesc;
  else [sortDesc, sortBy] = [false, columnName];
}

function toggleRowSelection(index: number) {
  if (!selection || !selection.length) selection = [index];
  else if (shiftState) { // Selection becomes a range including the highest, lowest and clicked row
    const points = [Math.max(...selection), Math.min(...selection), index];
    const [max, min] = [Math.max(...points), Math.min(...points)];
    selection = Array.from({length: max - min + 1}, (_, i) => i + min);
  } else if (ctrlState) // Toggle the presence of the row in the selection
    selection = selection.includes(index)
      ? selection.filter(i => i !== index)
      : selection =  [...selection, index];
  else selection = selection.includes(index) ? [] : [index]; // Toggle between selection of none and this row
  emit('selectionChanged');
}

function atPath(value: any, path: string): any {
  for (const segment of path.split('.'))
    value = (value ?? {} as any)[segment];
  return value;
}

let draggedRow = $ref(-1);
let draggedOverRow = $ref(-1);
function dragDropRow() {
  if (data) {
    const row = data[draggedRow];
    data.splice(draggedRow, 1);
    data.splice(draggedOverRow, 0, row);
    data = data;
    emit('draggedRow', draggedRow, draggedOverRow);
  }

  // Reset Drag & Remove Selection
  draggedRow = 0;
  draggedOverRow = 0;
  selection = [];
}
</script>

<template>
  <table>
    <thead>
      <tr>
        <th v-for="{heading, path} of columns" :key="path" @click="() => toggleSorting(path)">
          <div class="flex-row">
            {{ heading }}
            <template v-if="sort">
              <i-mdi-arrow-down v-if="path === sortBy && sortDesc"/>
              <i-mdi-arrow-up v-else-if="path === sortBy"/>
            </template>
          </div>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(row, index) in displayData" :key="index"
          :draggable="draggable"
          @click="() => toggleRowSelection(index)"
          @dblclick="() => emit('rowAction', index)"
          @dragstart="() => draggedRow = index"
          @dragenter="() => draggedOverRow = index"
          @dragend="() => dragDropRow()"
          :class="{
            'selected': (selection ?? []).includes(index),
            'dragged-over-before': index === draggedOverRow && draggedOverRow < draggedRow,
            'dragged-over-after': index === draggedOverRow && draggedOverRow > draggedRow,
          }">
        <td v-for="{path, component} of columns" :key="path">
          <component v-if="component" :is="component" v-bind="{[componentProp[component]]: atPath(row, path)}"/>
          <template v-else>{{ atPath(row, path) }}</template>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.dragged-over-before { border-top: 0.25rem solid var(--cl-fg); }
.dragged-over-after { border-bottom: 0.25rem solid var(--cl-fg); }

tr { cursor: grab; }
</style>