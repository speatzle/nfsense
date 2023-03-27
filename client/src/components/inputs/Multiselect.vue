<script setup lang="ts">
const props = defineModel<{
  options?: Record<any, any>,
  selection?: any[],
  search?: string,
}>();
let {options, selection, search} = $(props);

options ??= {
  bt1: "Boring Test Data 1",
  bt2: "Boring Test Data 2",
  bt3: "Boring Test Data 3",
  bt4: "Boring Test Data 4",
  bt5: "Boring Test Data 5",
  bt6: "Boring Test Data 6",
};
selection ??= ["bt1"];
search ??= "";

let open = $ref(false);
let navigated = $ref(0);
let input: HTMLElement | null = $ref(null);

const selected = $computed(() => selection?.length || 0);

function toggle(key: any) {
  if (selection?.includes(key)) selection?.splice(selection?.indexOf(key), 1);
  else selection?.push(key);
  input?.focus();
}

function onInputKeydown(e: KeyboardEvent) {
  switch (e.key) {
  case "Backspace":
  case "Delete":
    if (!selection || search !== "") break;
    if (navigated === 0) selection?.pop();
    else if (navigated > 0) navigated = 0;
    else {
      selection?.splice(navigated, 1);
      navigated = 0;
    }
    selection = selection;
    break;
  case "ArrowUp":
    navigated--;
    if (-navigated > selected) navigated = 0;
    break;
  case "ArrowDown":
    navigated++;
    if (navigated > Object.entries(options || {}).length) navigated = 0;
    break;
  case "Enter":
    if (!open) open = true;
    else if (navigated > 0)
      toggle(Object.entries(options || {})[navigated-1][0]);
    break;
  case "Escape":
    if (navigated !== 0) navigated = 0;
    else open = false;
    break;
  default: break;
  }
}
</script>
<template>
  <div :class="{'multiselect': 1, 'cl-secondary': 1, open}"
       @focusout="$event => open = ($event.currentTarget as HTMLElement).contains(($event.relatedTarget as HTMLElement))">
    <div class="head">
      <div class="selection">
        <div v-for="(key, index) of selection" :key="key" v-text="(options || {})[key]" @click="() => toggle(key)" :class="{navigated: selected + navigated === index}"/>
      </div>
      <div class="searchbar" @focusin="() => open = true">
        <button class="expand" @click="() => ($refs.input as any).focus()">
          <i-material-symbols-expand-circle-down-outline/>
        </button>
        <input placeholder="Search..." v-model="search" ref="input" @keydown="onInputKeydown"/>
      </div>
    </div>
    <Transition name="fade-fast">
      <div class="dropdown" v-if="open" tabindex="-1" @focusin="() => open = true">
        <div v-for="([key, option], index) in Object.entries(options || {})" :key="key" @click="() => toggle(key)" :class="{selected: selection?.includes(key), navigated: navigated === index + 1}">
          <i-material-symbols-check-box-outline v-if="selection?.includes(key)"/>
          <i-material-symbols-check-box-outline-blank v-else/>
          <div v-text="option"/>
        </div>
      </div>
    </Transition>
  </div>
</template>
<style scoped>
* { border: none; }
.selection, .searchbar, .dropdown {
  border: 1px solid var(--cl-fg);
}
.selection { border-bottom: none; }
.dropdown { border-top: none; }

.selection, .dropdown {
  flex-grow: 1;
  padding: 0.5rem;
  background-color: var(--cl-bg);
}

.selection {
  gap: 0.5rem;
}

.selection > div {
  background-color: var(--cl-bg-el);
}

:is(.selection, .dropdown) > div {
  flex-flow: row nowrap;
  gap: 0.5rem;
  cursor: pointer;
  padding: 0.25rem;
}
:is(.selection, .dropdown) > div > div {
  justify-content: center;
}
:is(.selection, .dropdown) > div.selected {
  background-color: var(--cl-bg-sl);
}
:is(.selection, .dropdown) > div:hover,
:is(.selection, .dropdown) > div.navigated {
  background-color: var(--cl-bg-hl);
}

.searchbar {
  flex-flow: row nowrap;
}

.searchbar > input {
  flex-grow: 1;
}

input {
  padding: 0.5rem;
}

.expand > svg { transition: all 0.2s ease-out; }
.open .expand > svg { transform: rotate(180deg); }

div:empty {
  display: none;
}
</style>