<!-- Base component that implements selecting single and multiple values from a list in a type-unsafe manner -->
<script setup lang="ts">
import { equals, isNullish, Index } from '../../util';
// --- Prop setup ---
const props = withDefaults(defineProps<{
  // Two-Way Bindings (v-model)
  modelValue?: any,
  search?: string,

  // One-Way Bindings
  multiple?: boolean,
  options?: Record<Index, {
    [key: Index]: any, // Allow additional properties for customization
    display?: string,
  }>,
}>(), {
  modelValue: null,
  search: "",
  multiple: false,
  options: () => ({}),
});
let { multiple, options } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: any): void,
  (e: 'update:search', value: string): void,
}>();

// Hook up two-way bindings
let modelValue = $ref(multiple ? props.modelValue ?? [] : props.modelValue);
watch(() => props.modelValue, (val: any) => { if (!equals(val, modelValue)) modelValue = val; }, { deep: true });
watch($$(modelValue), (val: any) => emit('update:modelValue', modelValue), { deep: true });
let search = $ref(props.search);
watch(() => props.search, (val: string) => { if (!equals(val, search)) search = val; }, { deep: true });
watch($$(search), (val) => emit('update:search', search), { deep: true });

// --- Everything Else  ---
let expanded = $ref(false);
let navigated = $ref(0);
let inputDiv: HTMLElement | null = $ref(null);
let input: HTMLElement | null = $ref(null);
let valueButton: HTMLElement | null = $ref(null);

const selCount = $computed(() => modelValue?.length || 0);

watch($$(multiple), () => modelValue = multiple ? [] : null );

function expand() {
  expanded = true;
  navigated = 0;
  focus();
}

function focus() {
  if (multiple || isNullish(modelValue)) input?.focus();
  else valueButton?.focus();
}

let skipFocusIn = false;
function focusIn() {
  if (skipFocusIn) return skipFocusIn = false;
  expand();
}

function toggle(key: any) {
  if (multiple) {
    const mv = modelValue as Index[];
    if (mv?.includes(key)) mv?.splice(mv?.indexOf(key), 1);
    else mv?.push(key);
    focus();
    return;
  }

  inputDiv?.focus();
  navigated = 0;
  if (modelValue === null || modelValue !== key) {
    modelValue = key;
    expanded = false;
  }
  else modelValue = null;
  skipFocusIn = true;
  setTimeout(focus, 0); // nextTick causes double fire on keydown.Enter, so defer to next event loop instance
}

function handleKeydown(e: KeyboardEvent) {
  switch (e.key) {
  case "Backspace":
  case "Delete":
    if (!modelValue || search !== "" || !multiple) break;
    if (navigated === 0) modelValue.pop();
    else if (navigated > 0) navigated = 0;
    else {
      modelValue.splice(navigated, 1);
      navigated = 0;
    }
    modelValue = modelValue;
    break;
  case "ArrowUp":
    navigated--;
    if (-navigated > selCount) navigated = 0;
    e.preventDefault(); // Prevent cursor from moving to the front/back
    break;
  case "ArrowDown":
    navigated++;
    if (navigated > Object.entries(options).length) navigated = 0;
    e.preventDefault(); // Prevent cursor from moving to the front/back
    break;
  case "Enter":
    if (!expanded) expand();
    else if (navigated > 0) {
      toggle(Object.entries(options)[navigated-1][0]);
    }
    break;
  case "Escape":
    if (navigated !== 0) navigated = 0;
    else expanded = false;
    break;
  default: break;
  }
}
</script>
<template>
  <div :class="{'multiselect': 1, 'cl-secondary': 1, expanded}" ref="inputDiv"
       @keydown="handleKeydown"
       @focusin="$event => { if (!inputDiv?.contains($event.relatedTarget as HTMLElement)) focusIn(); }"
       @focusout="$event => expanded = inputDiv?.contains($event.relatedTarget as HTMLElement) ?? false">
    <div class="head">
      <div class="selection" v-if="multiple" tabindex="-1">
        <div v-for="(key, index) of modelValue as Index[]" :key="key" v-text="options[key].display" :class="{navigated: selCount + navigated === index}"
             @click="() => toggle(key)"/>
      </div>
      <div class="searchbar">
        <div class="expand button" :tabindex="expanded ? undefined : -1">
          <i-material-symbols-expand-circle-down-outline width="1em" height="1em"/>
        </div>
        <input v-if="multiple || modelValue === null" placeholder="Search..." v-model="search" ref="input"/>
        <button v-else v-text="options[modelValue]?.display" ref="valueButton"
                @click="() => toggle(modelValue)"/>
      </div>
    </div>
    <Transition name="fade-fast">
      <div tabindex="-1" class="dropdown" v-if="expanded">
        <div v-for="([key, option], index) in Object.entries(options)" :key="key" :class="{selected: modelValue?.includes(key), navigated: navigated === index + 1}"
             @click="() => toggle(key)">
          <template v-if="multiple">
            <i-material-symbols-check-box-outline v-if="modelValue?.includes(key)" width="1em" height="1em"/>
            <i-material-symbols-check-box-outline-blank v-else width="1em" height="1em"/>
          </template>
          <div v-text="option.display"/>
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
.dropdown {
  border-top: none;
  overflow-y: auto;
  max-height: 10rem;
}

.selection, .dropdown {
  flex-grow: 1;
  padding: 0.5rem;
  gap: 0.5rem;
  background-color: var(--cl-bg);
}

.selection > div {
  background-color: var(--cl-bg-el);
}

:is(.selection, .dropdown) > div {
  flex-flow: row nowrap;
  padding: 0.25rem;
  gap: 0.5rem;
  cursor: pointer;
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

.searchbar { flex-flow: row nowrap; background: var(--cl-bg-hl) }
.expand { padding: 0px; }
.searchbar > :not(:first-child) { flex-grow: 1; justify-content: flex-start; }
input { padding: 0.25rem; outline: none; }
.expand > svg { transition: all 0.2s ease-out; }
.expanded .expand > svg { transform: rotate(180deg); }

div:empty { display: none; }
button { padding: 0.25rem; }
</style>