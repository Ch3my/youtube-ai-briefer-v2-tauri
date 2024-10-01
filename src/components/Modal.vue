<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  modelValue: boolean;
  title?: string;
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Modal Title'
});

const emit = defineEmits(['update:modelValue']);

const modalRef = ref<HTMLDialogElement | null>(null);

watch(() => props.modelValue, (newValue) => {
  if (newValue && modalRef.value) {
    modalRef.value.showModal();
  } else if (!newValue && modalRef.value) {
    modalRef.value.close();
  }
});

const closeModal = () => {
  emit('update:modelValue', false);
};
</script>

<template>
  <dialog ref="modalRef" @close="closeModal" class="p-0 bg-slate-800 text-white rounded-lg shadow-xl">
    <div class="flex flex-col w-full max-w-md">
      <div class="flex justify-between items-center p-4 border-b border-slate-600">
        <h3 class="text-lg font-semibold">{{ title }}</h3>
        <button @click="closeModal" class="text-slate-400 hover:text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div class="p-4">
        <slot></slot>
      </div>
      <div class="flex justify-end p-4 border-t border-slate-600">
        <slot name="footer">
          <button @click="closeModal" class="px-4 py-2 bg-slate-700 text-white rounded hover:bg-slate-600">
            Close
          </button>
        </slot>
      </div>
    </div>
  </dialog>
</template>

<style scoped>
dialog::backdrop {
  background-color: rgba(0, 0, 0, 0.5);
}
</style>