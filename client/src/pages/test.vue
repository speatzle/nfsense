<script setup lang="ts">
import { SearchProvider, Options } from '~/components/inputs/DropdownInput.vue';

const testValues: Options = {
  1: { display: 'Option 1' },
  2: { display: 'Option 2' },
  0: { display: 'Option 0' },
  a: { display: 'Option a' },
  z: { display: 'Option z' },
};
let vm: any = $ref({});

function genSP(indexIsChar: boolean): SearchProvider {
  return async (o) => {
    const all = Object.entries(testValues);
    const generatorSpecified = all.filter(x => indexIsChar === isNaN(parseInt(x[0])));
    const matching = generatorSpecified.filter(x => x[1].display?.includes(o.search));
    return Object.fromEntries(matching);
  };
}
</script>
<template>
  <div>
    <PageHeader title="Test Page"/>
    <SingleSelect :search-provider="genSP(true)" v-model="vm['Single']"/>
    <MultiSelect :search-provider="genSP(false)" v-model="vm['Multiple']"/>
    {{ vm }}
    <button @click="() => vm.Multiple = [1]">Click me</button>
    <button @click="() => vm.Multiple = [42]">Click me but EEEEVIL</button>
  </div>
</template>
