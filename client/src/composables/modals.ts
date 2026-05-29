import { Component } from "vue";

export type ModalDefinition = {
  id: string;
  is: Component;
  resolve: (value: unknown) => void;
  props: object;
};

const $modalStack: ModalDefinition[] = [];

function pushModal(is: Component, props: object = {}) {
  return new Promise((resolve) =>
    $modalStack.push({ is: markRaw(is), props, resolve, id: crypto.randomUUID() }),
  );
}
function popModal(value?: unknown) {
  $modalStack.pop()?.resolve(value);
}

export function useModals() {
  return { pushModal, popModal, modalStack: $$($modalStack) };
}
