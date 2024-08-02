<script setup lang="ts">
const props = withDefaults(defineProps<{
  data: match | undefined,
}>(), {
  data: undefined,
});
type port = { single?: { port: number }, range?: { start_port: number, end_port: number } };
type portOrAny = string | port;
type match = { source: portOrAny, destination: portOrAny };

const value = computed(() => {
  if (props.data === undefined) {
    return 'unknown';
  }
  return `${computePort(props.data.source)}->${computePort(props.data.destination)}`;
});

function computePort(port: portOrAny) {
  if (typeof port === 'string') {
    return 'any';
  } else if (port.single !== undefined) {
    return port.single.port;
  } else if (port.range != undefined){
    return `(${port.range.start_port}-${port.range.end_port})`;
  } else {
    return 'unknown';
  }
}

</script>
<template>
  {{ value }}
</template>
