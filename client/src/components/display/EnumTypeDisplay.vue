<script setup lang="ts">
import { variantOf } from '~/util';

const props = withDefaults(defineProps<{
  data: object | string,
}>(), {
  data: '',
});
</script>
<template>
  {{ variantOf(props.data) }}
</template>
