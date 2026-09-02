<script setup lang="ts">
import { usePendingChangeCount } from "~/api";

const $pendingChangeCount = $(usePendingChangeCount());
</script>
<template>
  <i-material-symbols-warning-outline v-if="$pendingChangeCount > 0" class="warning-icon"/>
  <div v-else/>
</template>
