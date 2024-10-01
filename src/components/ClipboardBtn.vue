<script setup lang="ts">
const emit = defineEmits(['clipboard-content']);

const handleClick = async () => {
  try {
    const text = await navigator.clipboard.readText();
    emit('clipboard-content', text);
  } catch (err) {
    console.error('Failed to read clipboard contents: ', err);
    // You might want to emit an error event here
    // emit('clipboard-error', 'Failed to read clipboard');
  }
};
</script>

<template>
  <button 
    type="button" 
    class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-500"
    @click="handleClick"
    title="Paste from clipboard"
  >
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      class="icon icon-tabler icon-tabler-clipboard" 
      width="24"
      height="24" 
      viewBox="0 0 24 24" 
      stroke-width="1.5" 
      stroke="currentColor" 
      fill="none" 
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path stroke="none" d="M0 0h24v24H0z" fill="none" />
      <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" />
      <path d="M9 3m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v0a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" />
    </svg>
  </button>
</template>