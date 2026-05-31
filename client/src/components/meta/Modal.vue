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
  border: 0.25rem solid var(--cl-bg-el);
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
  background: var(--cl-bg-el);

  & > button {
    padding: 0rem;
    &:hover {
      background: var(--cl-bg-hl);
    }
  }
}
.modal-footer {
  flex-wrap: wrap;
  justify-content: flex-end;
}
</style>
