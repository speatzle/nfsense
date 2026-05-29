<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
const { popModal } = useModals();
</script>
<template>
  <div class="modal-header flex-row gap">
    <h1 class="flex-grow" v-text="props.title" />
    <slot name="header" />
    <button @click="popModal()"><i-material-symbols-close /></button>
  </div>
  <div class="modal-body gap flex-grow"><slot /></div>
  <div class="modal-footer flex-row gap"><slot name="footer" /></div>
</template>
<style>
.modal-header {
  place-items: center;

  & > button {
    padding: 0.25rem;
  }
}
.modal-footer {
  padding-top: 0.5rem;
  flex-wrap: wrap;
  justify-content: flex-end;
}
</style>
