<!-- Wrapper component that sets "multiple" on DropdownInput to true and declares its type to be an array of any -->
<script setup lang="ts">
import { Options, MaybeSearchProvider } from "./input";
const props = withDefaults(
  defineProps<{
    // Two-Way Bindings (v-model)
    modelValue?: Index[];
    search?: string;
    options?: Options;

    // One-Way Bindings
    default?: Index[];
    searchProvider?: MaybeSearchProvider;
    placeholder?: string;
  }>(),
  {
    search: "",
    options: () => ({}),
    default: () => [],
    searchProvider: null,
    placeholder: "Search...",
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: any): void;
  (e: "update:search", value: string): void;
  (e: "update:options", value: Options): void;
}>();

// Hook up two-way bindings
let mVal = $ref(props.modelValue ?? props.default);
// TODO: Investigate Default Handling
syncModel(toRef(props, "modelValue"), $$(mVal), (v) => emit("update:modelValue", v), true);
let search = $ref(props.search);
syncModel(toRef(props, "search"), $$(search), (v) => emit("update:search", v));
let options = $ref(props.options);
syncModel(toRef(props, "options"), $$(options), (v) => emit("update:options", v), true);
</script>

<template>
  <DropdownInput
    v-model="mVal"
    :multiple="true"
    :options="options"
    :search-provider="searchProvider"
  />
</template>
