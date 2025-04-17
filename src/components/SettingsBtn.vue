<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import Modal from './Modal.vue';
import { invoke } from "@tauri-apps/api/core";

const isModalOpen = ref(false);

const openModal = () => {
  isModalOpen.value = true;
};

const availableModels = ref(["claude-3-5-haiku-20241022", "claude-3-7-sonnet-20250219", "gpt-4.1-mini-2025-04-14", "gpt-4.1-nano-2025-04-14"]);

const ragSearchTypes = ref(['similarity', 'mmr']);

const config = reactive({
  resumeModel: 'gpt-4.1-nano-2025-04-14',
  resumeChunkSize: 10000,
  condensaModel: 'gpt-4.1-nano-2025-04-14',
  ragModel: 'gpt-4.1-nano-2025-04-14',
  ragSearchType: 'similarity',
  ragSearchK: 5,
  ragChunkSize: 1000,
  useWhisper: 'no',
});

async function saveConfig() {
  await invoke("write_config", { jsonData: JSON.stringify(config) });
}

onMounted(async () => {
  const configFile = await invoke("read_config");
  Object.assign(config, configFile);
});

</script>

<template>
  <div>
    <button @click="openModal" type="button"
      class="flex items-center focus:outline-hidden font-medium rounded-sm text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">
      <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-settings" width="24" height="24"
        viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
        stroke-linejoin="round">
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path
          d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z" />
        <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" />
      </svg>
    </button>

    <Modal v-model="isModalOpen" title="Config">
      <div class="w-[40vw] grid grid-cols-2 gap-4 grid-rows-8 items-center">
        <label for="resumeModel" class="">Resume Model:</label>
        <select v-model="config.resumeModel" id="resumeModel" class="p-2 border rounded-sm">
          <option v-for="model in availableModels" :key="model" :value="model" class="text-slate-800">{{ model }}</option>
        </select>
        <label for="resumeChunkSize" class="mb-1">Resume Chunk Size:</label>
        <input v-model.number="config.resumeChunkSize" id="resumeChunkSize" type="number"
          class="p-2 border rounded-sm" />
        <label for="condensaModel" class="mb-1">Condensa Model:</label>
        <select v-model="config.condensaModel" id="condensaModel" class="p-2 border rounded-sm">
          <option v-for="model in availableModels" :key="model" :value="model" class="text-slate-800">{{ model }}</option>
        </select>
        <label for="ragModel" class="mb-1">RAG Model:</label>
        <select v-model="config.ragModel" id="ragModel" class="p-2 border rounded-sm">
          <option v-for="model in availableModels" :key="model" :value="model" class="text-slate-800">{{ model }}</option>
        </select>
        <label for="ragSearchType" class="mb-1">RAG Search Type:</label>
        <select v-model="config.ragSearchType" id="ragSearchType" class="p-2 border rounded-sm">
          <option v-for="type in ragSearchTypes" :key="type" :value="type" class="text-slate-800">{{ type }}</option>
        </select>
        <label for="ragSearchK" class="mb-1">RAG Search K:</label>
        <input v-model.number="config.ragSearchK" id="ragSearchK" type="number"
          class="p-2 border rounded-sm" />
        <label for="ragChunkSize" class="mb-1">RAG Chunk Size:</label>
        <input v-model.number="config.ragChunkSize" id="ragChunkSize" type="number"
          class="p-2 border rounded-sm" />
        <label for="useWhisper" class="mb-1">Use Whisper:</label>
        <select v-model="config.useWhisper" id="useWhisper" class="p-2 border rounded-sm">
          <option value="si" class="text-slate-800">Sí</option>
          <option value="no" class="text-slate-800">No</option>
        </select>
      </div>
      <button class="bg-slate-700 p-2.5 rounded-sm enabled:hover:bg-slate-600 flex-1 disabled:opacity-50"
        @click="saveConfig">Guardar</button>
    </Modal>
  </div>
</template>