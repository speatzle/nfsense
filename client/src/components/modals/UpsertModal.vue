<script setup lang="ts">
import { subsystems, type Entity } from "~/definitions";
const toast = useToast();

const props = defineProps<{
  subsystem: keyof typeof subsystems;
  entity: string;
  id?: string | number;
}>();
const $subsystem = $(computed(() => subsystems[props.subsystem]));
const $entity = $(computed(() => ($subsystem.entities as Record<string, Entity>)[props.entity]));
const $isEdit = $(computed(() => !isNullish(props.id)));

const { popModal } = useModals();

let $vm = {} as any; // TODO: Add proper type
let $loading = true;

async function load() {
  console.log({ LOOKATME: $isEdit });
  if (!$isEdit) return void ($loading = false);
  $loading = true;
  let res: any;
  if ($entity.idType == "Number")
    res = await apiCall(`${props.subsystem}.${props.entity}.get`, {
      id: (props.id as number) - 0,
    });
  else res = await apiCall(`${props.subsystem}.${props.entity}.get`, { id: props.id });

  if (res.Error === null) {
    console.debug("update data", res.Data);
    $vm = res.Data;
  } else console.debug("error", res);
  $loading = false;
}

async function upsert() {
  console.debug("value", $vm);
  let res: any;

  if (!$isEdit) res = await apiCall(`${props.subsystem}.${props.entity}.create`, $vm);
  else if ($entity.idType === "Number")
    res = await apiCall(`${props.subsystem}.${props.entity}.update`, {
      id: Number(props.id),
      thing: $vm,
    });
  else if (props.id === $vm.name || confirm("Do you want to change the name & all references?"))
    res = await apiCall(`${props.subsystem}.${props.entity}.update`, {
      id: props.id,
      thing: $vm,
    });
  else return;

  if (res.Error === null) {
    toast.success(`Updated ${$entity.name}`);
    popModal(res.Data);
  } else console.debug("error", res);
}

onMounted(async () => {
  if ($entity) load();
});
</script>
<template>
  <Modal :title="`${$isEdit ? 'Update' : 'Create'} ${$entity.name}`">
    <template v-if="$loading">Loading...</template>
    <template v-else-if="!$entity">No editType for this Entity</template>
    <template v-else>
      <NicerForm v-model="$vm" class="scroll" :fields="$entity.fields" />
      <pre class="pad" v-text="JSON.stringify($vm, null, 2)"/>
    </template>
    <template #footer>
      <button @click="popModal">Discard</button>
      <button @click="upsert" class="accent">Submit</button>
    </template>
  </Modal>
</template>
<style>
pre {
  --cl-z: 0;
  --cl-e: -1;
  background: var(--cl-bg);
  user-select: text;
}
</style>
