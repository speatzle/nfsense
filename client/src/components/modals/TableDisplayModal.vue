<script setup lang="ts">
const props = defineProps<{ data?: Record<string, any>; title?: string }>();
</script>
<template>
  <Modal :title="props.title ?? 'Data'">
    <table>
      <thead>
        <tr>
          <th>Field</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="[key, value] of Object.entries(props.data ?? {})">
          <td v-text="key" />
          <td v-text="value" />
        </tr>
      </tbody>
    </table>
  </Modal>
</template>
