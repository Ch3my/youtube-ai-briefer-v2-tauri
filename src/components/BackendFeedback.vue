<template>
    <div :class="alertClass" role="alert" v-show="type" class="flex gap-2 text-sm">
      <svg
        class="flex-shrink-0 inline w-4 h-4"
        aria-hidden="true"
        xmlns="http://www.w3.org/2000/svg"
        fill="currentColor"
        viewBox="0 0 20 20"
      >
        <path
          d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"
        />
      </svg>
      <p>
        {{ txt }}
      </p>
    </div>
  </template>
  
  <script setup>
  import { computed } from 'vue';
  
  const props = defineProps({
    txt: {
      type: String,
      required: true,
    },
    type: {
      type: String,
      default: 'info', // can be 'info', 'alert', or 'success'
      validator: (value) => ['info', 'alert', 'success', ''].includes(value),
    },
  });
  
  // Define computed property for class based on type
  const alertClass = computed(() => {
    const baseClass = 'flex items-center p-4 rounded-lg';
  
    if (props.type === 'info') {
      return `${baseClass} bg-sky-700 text-gray-300`;
    }
  
    if (props.type === 'alert') {
      return `${baseClass} bg-red-800`;
    }
  
    if (props.type === 'success') {
      return `${baseClass} bg-green-800`;
    }
  
    return baseClass;
  });
  </script>
  