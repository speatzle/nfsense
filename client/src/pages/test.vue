<script setup lang="ts">
import { SearchProvider, Options } from '../components/inputs/DropdownInput.vue';

const testValues: Options = {
  1: { display: 'Option 1' },
  2: { display: 'Option 2' },
  0: { display: 'Option 0' },
  a: { display: 'Option a' },
  z: { display: 'Option z' },
};
let vm: any = $ref({Multiple: [1]});

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
    <NicerForm :fields="{
      Single: { is: 'SingleSelect', label: 'SingleSelect', props: { options: testValues, searchProvider: genSP(true) } },
      Multiple: { is: 'MultiSelect', label: 'Multiselect', props: { options: testValues, searchProvider: genSP(false) } },
      adv: { is: 'Heading', props: { caption: 'Subsection' } },
      IP: { is: 'EnumInput', label: 'IP Address', props: { variants: {
        'dhcp': { display: 'DHCP' },
        'static-ipv4': {
          display: 'Static (IPV4)',
          fields: {
            ip: { is: 'TextBox', label: 'IP Address (CIDR)' },
            gw: { is: 'TextBox', label: 'Gateway Address' },
          },
        },
      }}},
      Subform: { is: 'NicerForm', props: { heading: 'Subform', fields: {
        Text: { is: 'TextBox', label: 'Text' },
        Subform2: { is: 'NicerForm', props: { heading: 'Subform2', fields: {
          Text: { is: 'TextBox', label: 'Text' },
        } } },
      } } },
    }" v-model="vm"/>
    {{ vm }}
    <button @click="() => { vm.Multiple = [1]; }">Click me</button>
    <button @click="() => { vm.Multiple = [42]; }">Click me but EEEEVIL</button>
    <button @click="() => { vm.IP = {'static-ipv4': {ip: '192.168.69.2/24', gw: '192.168.69.1' }} }">Click me but kinda pog wth</button>
  </div>
</template>
