<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
const { popModal } = useModals();
</script>
<template>
  <div class="modal-header flex-row gap">
    <h2 class="flex-grow" v-text="props.title" />
    <slot name="header" />
    <button @click="popModal()"><i-material-symbols-close /></button>
  </div>
  <div class="modal-body gap flex-grow"><slot /></div>
  <div class="modal-footer flex-row gap"><slot name="footer" /></div>
</template>
<style scoped>
:is(.modal-header, .modal-body, .modal-footer) {
  border: 2px solid var(--cl-bd);
  background-color: var(--cl-bg);
}
:is(.modal-body, .modal-footer) {
  border-top: none;
  padding: 0.5rem;
}
.modal-body {
  border-bottom: none;
  width: 100%;
  overflow: auto;
}
.modal-header {
  place-items: center;
  border-bottom: none;
  padding: calc(0.5rem);

  & > button {
    --button-padding: 0rem;
    --button-border: 0px;
  }
}
.modal-footer {
  flex-wrap: wrap;
  justify-content: flex-end;
}
</style>
