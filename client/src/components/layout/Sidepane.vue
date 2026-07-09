<script setup lang="ts">
const { modalStack, popModal } = useModals();
</script>
<template>
  <div class="sidepane" @click.self="popModal()">
    <TransitionGroup name="fade">
      <div class="pane" v-for="modal in modalStack" :key="modal.id">
        <component :is="modal.is" v-bind="modal.props ?? {}" />
      </div>
    </TransitionGroup>
  </div>
</template>
<style scoped>
.sidepane {
  display: grid;
  width: min(40rem, 50vw);
}

.pane {
  --cl-z: 1;
  display: grid;
  grid-template-rows: auto 1fr auto;
  width: min(40rem, 50vw);
  overflow: auto;
  grid-column: 1;
  grid-row: 1;
}
</style>
