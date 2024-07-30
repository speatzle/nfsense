<!-- Base component that implements selecting single and multiple values from a list in a type-unsafe manner -->
<script lang="ts">
// Types
export type Options = Record<Index, Option>;
export type Option = {
  [key: Index]: any, // Allow additional properties for customization
  display?: string,
};
export type SearchProvider = (opts: SearchOptions) => Promise<Options>;
export type MaybeSearchProvider = SearchProvider | null;
export type SearchOptions = {
  search: string,
  unknownKeys?: Index[],
  // parentData?: any,
};
</script>
<script setup lang="ts">
import { equals, isNullish, Index } from '../../util';
// --- Prop setup ---
const props = withDefaults(defineProps<{
  // Two-Way Bindings (v-model)
  modelValue?: any,
  search?: string,
  options?: Options,

  // One-Way Bindings
  multiple?: boolean,
  searchProvider?: MaybeSearchProvider,
  placeholder?: string,
}>(), {
  modelValue: null,
  search: '',
  options: () => ({}),
  multiple: false,
  searchProvider: null,
  placeholder: 'Search...',
});
let { multiple, searchProvider, placeholder } = $(props);

const emit = defineEmits<{
  (e: 'update:modelValue', value: any): void,
  (e: 'update:search', value: string): void,
  (e: 'update:options', value: Options): void,
}>();

// Local Variables for Two-Way bindings
let modelValue = $ref(multiple ? props.modelValue ?? [] : props.modelValue);
let search = $ref(props.search);
let options = $ref(props.options);
// Sync from v-model
onMounted(() => {
  watch(() => props.modelValue, async (val) => {
    if (equals(val, modelValue)) return;
    if (isNullish(val)) return modelValue = val; // Cant be unknown

    // Run search provider if key unknown, log and reject if still so
    let knownKeys = Object.keys(options);
    if (multiple) {
      let unknownKeys = (val as Index[]).filter(key => !knownKeys.includes(key.toString()));
      if (!unknownKeys.length) return modelValue = val;

      await performSearch(unknownKeys);
      knownKeys = Object.keys(options);
      unknownKeys = (val as Index[]).filter(key => !knownKeys.includes(key.toString()));
      for (let key of unknownKeys) console.warn('Unknown key in DropdownInput:', key/*, options*/);
      return modelValue = (val as Index[]).filter(key => knownKeys.includes(key.toString()));
    }
    if (!knownKeys.includes(val.toString())) {
      await performSearch([val]);
      knownKeys = Object.keys(options);
      if (!knownKeys.includes(val.toString()))
        return console.warn('Unknown key in DropdownInput:', val/*, options*/);
    }
    modelValue = val;
  }, { deep: true, immediate: true });
  watch(() => props.search, (val) => { if (!equals(val, search)) search = val; }, { deep: true });
  watch(() => props.options, (val) => { if (!equals(val, options)) options = val; }, { deep: true });
});
// Sync to v-model
watch($$(modelValue), () => emit('update:modelValue', modelValue), { deep: true });
watch($$(search), () => {
  emit('update:search', search);
  expand();
}, { deep: true });
watch($$(options), () => emit('update:options', options), { deep: true });

watch($$(multiple), () => modelValue = multiple ? [] : null );

// --- Everything Else  ---
let expanded = $ref(false);
let navigated = $ref(0);
let inputDiv: HTMLElement | null = $ref(null);
let input: HTMLElement | null = $ref(null);
let valueButton: HTMLElement | null = $ref(null);

const selCount = $computed(() => modelValue?.length || 0);

// Run search provider whenever the search changes, if supplied
watch($$(search), async (val, oldVal) => {
  if (val === oldVal) return;
  await performSearch();
});

async function performSearch(unknownKeys?: Index[]) {
  if (searchProvider !== null)
    options = await searchProvider({ search, unknownKeys });
}

async function expand() {
  performSearch();
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
    if (mv?.some((x: Index) => x == key)) mv?.splice(mv?.indexOf(key), 1);
    else mv?.push(key);
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
  // nextTick causes double fire on keydown.Enter, so defer to next event loop instance
  setTimeout(() => {
    focus();
    expanded = modelValue === null;
  }, 0);
}

function handleKeydown(e: KeyboardEvent) {
  switch (e.key) {
  case 'Backspace':
  case 'Delete':
    if (!modelValue || search !== '' || !multiple) break;
    if (navigated === 0) modelValue.pop();
    else if (navigated > 0) navigated = 0;
    else {
      modelValue.splice(navigated, 1);
      navigated = 0;
    }
    modelValue = modelValue;
    break;
  case 'ArrowUp':
    navigated--;
    if (-navigated > selCount) navigated = 0;
    e.preventDefault(); // Prevent cursor from moving to the front/back
    break;
  case 'ArrowDown':
    navigated++;
    if (navigated > Object.entries(options).length) navigated = 0;
    e.preventDefault(); // Prevent cursor from moving to the front/back
    break;
  case 'Enter':
    if (!expanded) expand();
    else if (navigated > 0) toggle(Object.entries(options)[navigated-1][0]);
    break;
  case 'Escape':
    if (navigated !== 0) navigated = 0;
    else expanded = false;
    break;
  default: break;
  }
}
</script>
<template>
  <div ref="inputDiv" :class="{'multiselect': 1, 'cl-secondary': 1, expanded}"
       @keydown="handleKeydown"
       @focusin="$event => { if (!inputDiv?.contains($event.relatedTarget as HTMLElement)) focusIn(); }"
       @focusout="$event => expanded = inputDiv?.contains($event.relatedTarget as HTMLElement) ?? false">
    <div class="head">
      <div v-if="multiple" class="selection">
        <div v-for="(key, index) of modelValue as Index[]" :key="key" :class="{navigated: selCount + navigated === index}" @click="() => toggle(key)"
             v-text="options[key].display"/>
      </div>
      <div class="searchbar">
        <div class="expand button" :tabindex="expanded ? undefined : -1">
          <i-material-symbols-expand-circle-down-outline width="1em" height="1em"/>
        </div>
        <input v-if="multiple || modelValue === null" ref="input" v-model="search" :placeholder="placeholder" @click="expand"/>
        <button v-else ref="valueButton" @click="() => toggle(modelValue)"
                v-text="options[modelValue]?.display"/>
      </div>
    </div>
    <Transition name="fade-fast">
      <div v-if="expanded" tabindex="-1" class="dropdown">
        <div v-for="([key, option], index) in Object.entries(options)" :key="key"
             :class="{selected: multiple ? modelValue?.some((x: Index) => x == key) : key == modelValue, navigated: navigated === index + 1}"
             @click="() => toggle(key)">
          <template v-if="multiple">
            <i-material-symbols-check-box-outline v-if="modelValue?.some((x: Index) => x == key)" width="1em" height="1em"/>
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