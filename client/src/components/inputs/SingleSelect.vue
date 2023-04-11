<!-- Wrapper component that sets "multiple" on DropdownInput to false and declares its type to be an Index -->
<script setup lang="ts">
import { Index } from "./DropdownInput.vue";
import { equals } from "../../util";
const props = withDefaults(defineProps<{
  // Two-Way Bindings (v-model)
  modelValue?: Index | null,
  search?: string,

  // One-Way Bindings
  options?: Record<Index, {
    [key: Index]: any, // Allow additional properties for customization
    display?: string,
  }>,
}>(), {
  modelValue: null,
  search: "",
  options: () => ({}),
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: any): void,
  (e: 'update:search', value: string): void,
}>();

// Hook up two-way bindings
let modelValue: Index[] | null = $ref(null);
watch(() => props.modelValue, (val: any) => { if (!equals(val, modelValue)) modelValue = val; }, { deep: true });
watch($$(modelValue), (val: any) => { if(!equals(val, props.modelValue)) emit('update:modelValue', modelValue); }, { deep: true });
let search = $ref(props.search);
watch(() => props.search, (val: string) => { if (!equals(val, search)) search = val; }, { deep: true });
watch($$(search), (val) => { if(!equals(val, props.search)) emit('update:search', search); }, { deep: true });

let { options } = $(props);
</script>
<template>
  <DropdownInput :multiple="false" :options="options" v-model="modelValue"/>
</template>
