<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    data: unknown;
    unit?: "bytes" | "number";
  }>(),
  { unit: "number" },
);

function format(n: unknown): string {
  const value = Number(n);
  if (!Number.isFinite(value) || value < 0) return "N/A";

  if (props.unit === "bytes") return formatBytes(value);
  return formatNumber(value);
}

function formatBytes(value: number): string {
  if (value === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB", "PB"];
  return formatSI(value, units);
}

function formatNumber(value: number): string {
  if (value === 0) return "0";
  const units = ["", "K", "M", "G", "T", "P"];
  return formatSI(value, units);
}

function formatSI(value: number, units: string[]): string {
  const base = 1000;
  const unitIndex = Math.min(Math.floor(Math.log(value) / Math.log(base)), units.length - 1);
  const scaled = value / Math.pow(base, unitIndex);
  const formatted =
    unitIndex === 0
      ? String(Math.round(scaled))
      : scaled.toFixed(2).replace(/\.?0+$/, "");
  return `${formatted} ${units[unitIndex]}`.trim();
}
</script>

<template>
  {{ format(props.data) }}
</template>
