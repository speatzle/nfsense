<!-- Wrapper component that sets "multiple" on DropdownInput to false and declares its type to be an Index -->
<script setup lang="ts">
import { Options, MaybeSearchProvider } from "./input";
const props = withDefaults(
  defineProps<{
    // Two-Way Bindings (v-model)
    modelValue?: MaybeIndex;
    default?: MaybeIndex;
    search?: string;
    options?: Options;

    // One-Way Bindings
    searchProvider?: MaybeSearchProvider;
    placeholder?: string;
  }>(),
  {
    default: null,
    search: "",
    options: () => ({}),
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
let $mVal = props.modelValue !== undefined ? props.modelValue : props.default;
syncModel(toRef(props, "modelValue"), $$($mVal), (v) => emit("update:modelValue", v), true);
let $search = props.search;
syncModel(toRef(props, "search"), $$($search), (v) => emit("update:search", v));
let $options = props.options;
syncModel(toRef(props, "options"), $$($options), (v) => emit("update:options", v), true);
</script>

<template>
  <DropdownInput
    v-model="$mVal"
    :multiple="false"
    :options="$options"
    :search-provider="searchProvider"
  />
</template>
