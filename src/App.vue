<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import Greet from "./components/Greet.vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Message {
  type: string;
  content: string;
  timestamp: number;
}

const socket = ref<WebSocket | null>(null);
const messages = ref<Message[]>([]);
const inputMessage = ref('');
const isConnected = ref(false);

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
      console.log('Received message:', jsonMessage);
      messages.value.push(jsonMessage);
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
    // Attempt to reconnect after 5 seconds
    setTimeout(connectWebSocket, 5000);
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
  await invoke("on_exit");
}
</script>

<template>
  <div class="flex h-screen bg-slate-800">
    <!-- First column (smallest) -->
    <div class="w-1/5 p-4 flex gap-2 flex-col">
      <input v-model="inputMessage" type="text"
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white" 
        placeholder="Enter your message" />
      <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600" @click="sendMessage">Send Message</button>
      <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600" @click="close">Close</button>
      <p class="text-white">Connection status: {{ isConnected ? 'Connected' : 'Disconnected' }}</p>
    </div>

    <!-- Second column (largest) -->
    <div class="w-3/5 p-4 ml-4 overflow-y-auto">
      <h2 class="text-white mb-4">Received Messages:</h2>
      <ul>
        <li v-for="(message, index) in messages" :key="index" class="text-white mb-2">
          {{ JSON.stringify(message) }}
        </li>
      </ul>
    </div>

    <!-- Third column (medium size) -->
    <div class="w-1/4 p-4 ml-4 flex flex-col gap-4">
      <textarea
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white h-full"></textarea>
      <input type="text"
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white">
      <div class="flex justify-between gap-2">
        <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600 flex-2">Save</button>
        <button class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600 flex-1">Enviar</button>
      </div>
    </div>
  </div>
  <Greet />
</template>

<style scoped></style>