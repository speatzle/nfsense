<script setup lang="ts">
import { useKeyModifier } from '@vueuse/core';

const shiftState = $(useKeyModifier('Shift'));
const ctrlState = $(useKeyModifier('Control'));

const props = defineModel<{
  columns?: {
    heading: string,
    path: string,
    component?: Component,
  }[],
  data?: Record<string, any>[],
  sort?: boolean,
  sortSelf?: boolean,
  sortBy?: string,
  sortDesc?: boolean,
}>();
let { columns, data, sort, sortSelf, sortBy, sortDesc } = $(props);

const emit = defineEmits<{
  (event: 'rowAction', index: number): void,
  (event: 'selectionChanged'): void
}>();

let selection = $ref([] as number[]);

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
  else {
    sortDesc = false;
    sortBy = columnName;
  }
}

function rowSelection(index: number) {
  if (shiftState) {
    if (selection.length === 0) {
      selection = [index];
    } else {
      let last = selection[selection.length-1];
      let start = index;
      let end = last;
      if (last < start) {
        start = last;
        end = index;
      }
      for (let i = start; i <= end; i++ ) {
        if (!selection.includes(i)) {
          selection =  [...selection, i];
        }
      }
    }
  } else if (ctrlState) {
    if (selection.includes(index)) {
      // remove if already exists
      selection.splice(selection.indexOf(index), 1);
    } else {
      selection =  [...selection, index];
    }
  } else {
    if (selection.includes(index)) {
      selection = [];
    } else {
      selection = [index];
    }
  }
  emit("selectionChanged");
}

function atPath(value: any, path: string): any {
  for (const segment of path.split(".")) {
    value = value[segment];
  }
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
  }
  // Reset drag data
  draggedRow = 0;
  draggedOverRow = 0;
  // Kill selection
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
          draggable="true"
          @click="() => rowSelection(index)"
          @dblclick="() => emit('rowAction', index)"
          @dragstart="() => draggedRow = index"
          @dragenter="() => draggedOverRow = index"
          @dragend="() => dragDropRow()"
          :class="{
            'selected': selection.includes(index),
            'dragged-over-before': index === draggedOverRow && draggedOverRow < draggedRow,
            'dragged-over-after': index === draggedOverRow && draggedOverRow > draggedRow,
          }">
        <td v-for="{path, component} of columns" :key="path">
          {{ component ? "" : atPath(row, path) }}
          <component v-if="component" :is="component"/>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.selected {
  background-color: red;
}

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