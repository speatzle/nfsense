<script setup lang="ts">
import type { Fields, ActionCallback } from "./input";
const props = withDefaults(
  defineProps<{
    // Two-Way Bindings
    modelValue?: Record<Index, any>;
    default?: Record<Index, any>;

    // One-Way Bindings
    fields?: Fields;
    heading?: string;
    headingLevel?: number;
  }>(),
  {
    default: () => ({}),
    fields: () => ({}),
    heading: "",
    headingLevel: 2,
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: Record<Index, any>): void;
}>();

// Local Variables for Two-Way bindings
let $modelValue = props.modelValue ?? props.default;
// Sync from v-model
onMounted(() => {
  watch(
    () => props.modelValue,
    (val) => {
      if (equals(val, $modelValue)) return;
      $modelValue = Object.assign({}, val ?? props.default);
    },
    { deep: true, immediate: true },
  );
});
// Sync to v-model
watch(
  $$($modelValue),
  (val) => {
    if (equals(val, props.modelValue)) return;
    emit("update:modelValue", Object.assign({}, val));
  },
  { deep: true, immediate: true },
);

function triggerAction(index: string, callback?: ActionCallback) {
  callback?.(index, $$($modelValue));
}
</script>

<template>
  <div
    :class="{
      form: 1,
      'has-actions': Object.entries(props.fields).some(([_, field]) => field.actions?.length),
    }"
  >
    <component :is="`h${headingLevel}`" v-if="heading">{{ heading }}</component>
    <div class="form inner-form">
      <template v-for="[index, field] of Object.entries(props.fields)" :key="index">
        <label v-if="field.label && field.is !== 'EnumInput'" v-text="field.label" />
        <component
          :is="field.is"
          v-model="$modelValue[index]"
          :default="props.default[index]"
          v-bind="
            field.is === 'EnumInput'
              ? Object.assign({ label: field.label }, field.props)
              : field.props
          "
        />
        <div class="actions flex-row" v-if="field.actions ?? []">
          <button
            v-for="action of field.actions?.filter((a) => !a.when || a.when(index, $modelValue))"
            @click="triggerAction(index, action.callback)"
          >
            <component v-if="action.icon" :is="action.icon" />
            <template v-else>{{ action.name }}</template>
          </button>
        </div>
      </template>
    </div>
  </div>
</template>
<style>
.form.has-actions {
  grid-template-columns: auto 1fr auto;
}
.actions {
  gap: 0.25rem;
  align-self: start;
  & > button {
    padding: 0.25rem;
    & > svg {
      padding: 0px;
      margin: -0.25rem;
    }
  }
}
</style>
