<script setup lang="ts">
import { computed } from 'vue';
interface Props {
  isConnected: boolean;
}
const props = defineProps<Props>();

const statusClass = computed(() => props.isConnected ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' : 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300');
const dotClass = computed(() => props.isConnected ? 'bg-green-500' : 'bg-red-500');
const statusText = computed(() => props.isConnected ? 'Backend Connected' : 'Backend Disconnected');
</script>

<template>
  <span :class="['inline-flex', 'items-center', 'text-xs', 'font-medium', 'px-2.5', 'py-0.5', 'rounded-full', statusClass]">
    <span :class="['w-2', 'h-2', 'me-1', 'rounded-full', dotClass]"></span>
    {{ statusText }}
  </span>
</template>