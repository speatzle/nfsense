<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
}>();

const $apiPath = $(computed(() => `${props.subsystem}.${props.entity}`));

const toast = useToast();

async function resetCounter(entry: any) {
  const name = entry.name;
  const res = await apiCall(`${$apiPath}.reset_counter`, { name });
  if (res.Error === null) {
    toast.success("Counter reset");
  } else console.debug("error", res);
}

</script>
<template>
  <TablePage :subsystem :entity :actions="[{
    caption: 'Reset Counter',
    target: 'single',
    callback: resetCounter,
  }]"/>
</template>
