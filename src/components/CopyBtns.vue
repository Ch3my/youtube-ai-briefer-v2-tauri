<script setup lang="ts">
import { defineProps } from 'vue';
import { useClipboard } from '../composables/useClipboard';

defineProps<{
    originalTranscript: string;
    originalNotes: string[];
    mainContent: string;
}>();

const emit = defineEmits(['copied']);

const { copyTextToClipboard } = useClipboard();

const handleCopy = async (content: string, type: string) => {
    await copyTextToClipboard(content);
    let msg = ""
    if (type == "transcript") {
        msg = "Transcript copiado correctamente"
    }
    if (type == "notes") {
        msg = "Notas copiadas correctamente"
    }
    if (type == "markdown") {
        msg = "Documento final copiado correctamente"
    }
    emit('copied', msg);
    // You can add some user feedback here, like a toast notification
};
</script>

<template>
    <div class="flex gap-4">
        <button type="button" @click="handleCopy(originalTranscript, 'transcript')" title="Copiar Transcript Original"
            class="flex items-center focus:outline-none font-medium rounded text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">
            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-record-mail" width="44"
                height="44" viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
                stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <path d="M7 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
                <path d="M17 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
                <path d="M7 15l10 0" />
            </svg>
        </button>
        <button type="button" @click="handleCopy(originalNotes.join('\n\n'), 'notes')"
            title="Copiar Notas de cada seccion"
            class="flex items-center  focus:outline-none font-medium rounded text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">
            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-notes" width="44" height="44"
                viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
                stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <path d="M5 3m0 2a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2z" />
                <path d="M9 7l6 0" />
                <path d="M9 11l6 0" />
                <path d="M9 15l4 0" />
            </svg>
        </button>
        <button type="button" @click="handleCopy(mainContent, 'markdown')" title="Copiar Markdown de documento final"
            class="flex items-center focus:outline-none font-medium rounded text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">
            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-markdown" width="44" height="44"
                viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
                stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <path d="M3 5m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" />
                <path d="M7 15v-6l2 2l2 -2v6" />
                <path d="M14 13l2 2l2 -2m-2 2v-6" />
            </svg>
        </button>
    </div>
</template>