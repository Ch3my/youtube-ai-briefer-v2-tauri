<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import BackendFeedback from "./components/BackendFeedback.vue";
import { invoke } from "@tauri-apps/api/tauri";

const socket = ref<WebSocket | null>(null);
const videoUrl = ref('');
const mainContent = ref('');
const isConnected = ref(false);
const backedFeedback = ref({
  type: null,
  txt: null
})

const connectWebSocket = () => {
  console.log('Attempting to connect to WebSocket...');
  socket.value = new WebSocket('ws://localhost:12345');

  socket.value.onopen = () => {
    console.log('WebSocket connection established');
    isConnected.value = true;
  };

  socket.value.onmessage = (event) => {
    try {
      const jsonMessage = JSON.parse(event.data);
      processBackendMessage(jsonMessage)
    } catch (error) {
      console.error('Error parsing incoming message:', error);
    }
  };

  socket.value.onerror = (error) => {
    console.error('WebSocket error:', error);
  };

  socket.value.onclose = (event) => {
    console.log('WebSocket connection closed', event.reason);
    isConnected.value = false;
    // Attempt to reconnect after x seconds
    setTimeout(connectWebSocket, 3000);
  };
};

const sendMessage = () => {
  if (socket.value && socket.value.readyState === WebSocket.OPEN) {
    const messageToSend: Message = {
      type: 'user_message',
      content: inputMessage.value,
      timestamp: Date.now()
    };
    console.log('Sending message:', messageToSend);
    socket.value.send(JSON.stringify(messageToSend));
    inputMessage.value = '';
  } else {
    console.error('WebSocket is not connected. Current state:', socket.value?.readyState);
    if (!isConnected.value) {
      console.log('Attempting to reconnect...');
      connectWebSocket();
    }
  }
};

const closeWebSocket = () => {
  if (socket.value) {
    console.log('Closing WebSocket connection');
    socket.value.close();
  }
};

onMounted(() => {
  connectWebSocket();
});

onUnmounted(() => {
  closeWebSocket();
});

async function close() {
  let result = await invoke("fetch_logs");
  console.log(result);
}

function processBackendMessage(jsonMessage) {
  if (jsonMessage.action == "notasDetalladas") {
    backedFeedback.value.type = "success"
    backedFeedback.value.txt = jsonMessage.msg
    mainContent.value = jsonMessage.notasDetalladas
  }
  if (jsonMessage.action == "ragAnswer") {
    backedFeedback.value.type = "success"
    backedFeedback.value.txt = jsonMessage.msg
  }
  if (jsonMessage.action == "message" && jsonMessage.msgCode == "info") {
    backedFeedback.value.type = "info"
    backedFeedback.value.txt = jsonMessage.msg
  }
  if (jsonMessage.action == "message" && jsonMessage.msgCode == "useWhisper") {
    backedFeedback.value.type = "info"
    backedFeedback.value.txt = jsonMessage.msg
  }
}
</script>

<template>
  <div class="flex h-screen ">
    <!-- First column (smallest) -->
    <div class="w-3/12 p-4 flex gap-2 flex-col">
      <div class="flex justify-between">
        <span v-if="isConnected"
          class="inline-flex items-center bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-green-900 dark:text-green-300">
          <span class="w-2 h-2 me-1 bg-green-500 rounded-full"></span>
          Backend Connected
        </span>
        <span v-else
          class="inline-flex items-center bg-red-100 text-red-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-red-900 dark:text-red-300">
          <span class="w-2 h-2 me-1 bg-red-500 rounded-full"></span> Backend Disconnected
        </span>
        <button type="button"
          class="flex items-center text-white  focus:outline-none focus:ring-4  font-medium rounded-lg text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">

          <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-settings" width="24" height="24"
            viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
            stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path
              d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z" />
            <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" />
          </svg> 
        </button>

      </div>
      <BackendFeedback :type="backedFeedback.type" :txt="backedFeedback.type" />
      <input v-model="videoUrl" type="text"
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white"
        placeholder="Youtube URL" />
      <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600" @click="sendMessage">Procesar</button>
    </div>

    <!-- Second column (largest) -->
    <div class="w-6/12 p-4 ml-4 overflow-y-auto">
      <!-- TODO MarkDown Procesor -->
      {{ mainContent }}
    </div>

    <!-- Third column (medium size) -->
    <div class="w-3/12 p-4 ml-4 flex flex-col gap-4">
      <textarea
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white h-full"></textarea>
      <input type="text"
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white">
      <div class="flex justify-between gap-2">
        <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600 flex-2">R</button>
        <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600 flex-1">Enviar</button>
      </div>
    </div>
  </div>
</template>

<style scoped></style>