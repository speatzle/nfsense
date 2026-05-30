<script setup lang="ts">
const { modalStack, popModal } = useModals();
</script>
<template>
  <Transition name="fade">
    <div class="popups-container" @click.self="popModal()" v-if="modalStack.length">
      <TransitionGroup name="fade">
        <div class="popup" v-for="modal in modalStack" :key="modal.id">
          <component :is="modal.is" v-bind="modal.props ?? {}" />
        </div>
      </TransitionGroup>
    </div>
  </Transition>
</template>
<style scoped>
.popups-container {
  position: absolute;
  inset: -100%;
  display: grid;
  place-items: center;
  background: #00000080;
}

.popup {
  background: var(--cl-bg);
  width: calc(min(100vw, 50rem) - 5vw);
  height: calc(min(100vh, 50rem) - 5vh);
  overflow-y: auto;
  grid-column: 1;
  grid-row: 1;
}
</style>
