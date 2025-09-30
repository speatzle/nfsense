<!-- Wrapper component that sets "multiple" on DropdownInput to false and declares its type to be an Index -->
<script setup lang="ts">
import { equals, MaybeIndex } from '../../util';
import { Options, MaybeSearchProvider } from './input';
const props = withDefaults(defineProps<{
  // Two-Way Bindings (v-model)
  modelValue?: MaybeIndex,
  search?: string,
  options?: Options,

  // One-Way Bindings
  searchProvider?: MaybeSearchProvider,
  placeholder?: string,
}>(), {
  modelValue: null,
  search: '',
  options: () => ({}),
  searchProvider: null,
  placeholder: 'Search...',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: any): void,
  (e: 'update:search', value: string): void,
  (e: 'update:options', value: Options): void,
}>();

// Hook up two-way bindings
let modelValue: MaybeIndex = $ref(null);
watch(() => props.modelValue, (val) => { if (!equals(val, modelValue)) modelValue = val; }, { deep: true });
watch($$(modelValue), (val) => { if(!equals(val, props.modelValue)) emit('update:modelValue', modelValue); }, { deep: true });
let search = $ref(props.search);
watch(() => props.search, (val) => { if (!equals(val, search)) search = val; }, { deep: true });
watch($$(search), (val) => { if(!equals(val, props.search)) emit('update:search', search); }, { deep: true });
let options = $ref(props.options);
watch(() => props.options, (val) => { if (!equals(val, options)) options = val; }, { deep: true });
watch($$(options), (val) => { if(!equals(val, props.options)) emit('update:options', options); }, { deep: true });

</script>

<template>
  <DropdownInput v-model="modelValue" :multiple="false" :options="options" :search-provider="searchProvider"/>
</template>
