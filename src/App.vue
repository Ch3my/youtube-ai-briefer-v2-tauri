<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import BackendFeedback from "./components/BackendFeedback.vue";
import VideoInfo from "./components/VideoInfo.vue";
// import { invoke } from "@tauri-apps/api/tauri";
import VueMarkdown from 'vue-markdown-render'
// @ts-ignore
import { getYoutubeMediaInfo } from "./youtube-helper"

const socket = ref<WebSocket | null>(null);
const videoUrl = ref('');
const mainContent = ref('');
const videoTitle = ref('');
const videoImg = ref('');
const videoDescription = ref('');
const isConnected = ref(false);
const processBtnDisabled = ref(false);
const queryRagBtnDisabled = ref(false);
const feedbackType = ref('')
const feedbackText = ref('')
const ragQuestion = ref('')
const ragContext = ref('')
const ragChat = ref<RagChatItem[]>([]);

const whisperConfirmed = ref(false)

type Context = {
  source: string;
  content: string;
};

type RagChatItem = {
  answer: string;
  context?: Context[];
  src: string
};
const chatBox = ref<HTMLElement | null>(null);
watch(ragChat, async () => {
  // Es necesario esperar nextTick para que el contenido ya este dentro del div
  // y podamos hacer scroll correctamente
  await nextTick();
  if (chatBox.value != null) {
    chatBox.value.scrollTop = chatBox.value.scrollHeight;
  }
}, { deep: true });

const connectWebSocket = () => {
  console.log('Attempting to connect to WebSocket...');
  socket.value = new WebSocket('ws://localhost:12345');

  socket.value.onopen = () => {
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

function sendMessage(msg: any): void {
  if (socket.value && socket.value.readyState === WebSocket.OPEN) {
    socket.value.send(JSON.stringify(msg));
  } else {
    console.error('WebSocket is not connected. Current state:', socket.value?.readyState);
    if (!isConnected.value) {
      console.log('Attempting to reconnect...');
      connectWebSocket();
    }
  }
};

function buildVideoData() {
  feedbackType.value = "info"
  feedbackText.value = "Procesamiento iniciado"
  processBtnDisabled.value = true
  mainContent.value = ""
  fetchMetaYT(videoUrl.value)
  sendMessage({
    whisperConfirmed: whisperConfirmed.value,
    url: videoUrl.value,
    action: "build"
  })
}

onMounted(() => {
  connectWebSocket();
});

onUnmounted(() => {
  if (socket.value) {
    console.log('Closing WebSocket connection');
    socket.value.close();
  }
});

async function fetchMetaYT(url: string = "https://www.youtube.com/watch?v=dQw4w9WgXcQ") {
  let result = await getYoutubeMediaInfo(url)
  if (!result) {
    console.log("Error al obtener datos del Video");
    return
  }
  videoTitle.value = result.title
  videoDescription.value = result.description.slice(0, 100)
  videoImg.value = result.thumbnailUrl
}

function processBackendMessage(jsonMessage: any) {
  if (jsonMessage.action == "notasDetalladas") {
    feedbackType.value = "success"
    feedbackText.value = "Procesamiento completado"
    mainContent.value = jsonMessage.notasDetalladas
    processBtnDisabled.value = false
  }
  if (jsonMessage.action == "ragAnswer") {
    queryRagBtnDisabled.value = false
    ragChat.value.push({
      answer: jsonMessage.answer,
      context: jsonMessage.context,
      src: "NIA"
    })
  }
  if (jsonMessage.action == "message" && jsonMessage.msgCode == "info") {
    feedbackType.value = "info"
    feedbackText.value = jsonMessage.msg
  }
  if (jsonMessage.action == "message" && jsonMessage.msgCode == "useWhisper") {
    feedbackType.value = "info"
    feedbackText.value = jsonMessage.msg
  }
}

function ragQuery() {
  queryRagBtnDisabled.value = true
  sendMessage({
    query: ragQuestion.value,
    action: "query"
  })
  ragChat.value.push({
    answer: ragQuestion.value,
    src: "USER"
  })
  ragQuestion.value = ""
}

function showContext() {
  // Filtramos el ultimo mensaje de NIA 
  const lastNiaAnswer = ragChat.value
    .filter(item => item.src === "NIA")  // Filter items where src is "NIA"
    .pop();  // Get the last matching item

  if (!lastNiaAnswer) {
    return
  }
  if (!lastNiaAnswer.context) {
    return
  }
  ragContext.value = ""
  for (const a of lastNiaAnswer.context) {
    ragContext.value += `Source: ${a.source}\n`
    ragContext.value += `${a.content}\n\n`
  }
}
</script>

<template>
  <div class="grid h-screen p-4 gap-4" style="grid-template-columns: 1fr 4fr 2fr;">
    <!-- First column -->
    <div class="flex gap-2 flex-col justify-between">
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
      <VideoInfo :video-title="videoTitle" :video-description="videoDescription" :video-img="videoImg" />
      <div class="flex flex-col gap-2">
        <BackendFeedback :type="feedbackType" :txt="feedbackText" />
        <input v-model="videoUrl" type="text"
          class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 text-white"
          placeholder="Youtube URL" />
        <div class="flex gap-2">
          <button class="bg-slate-700 text-white p-2.5 rounded enabled:hover:bg-slate-600 flex-1 disabled:opacity-50"
            @click="buildVideoData" :disabled="processBtnDisabled || !isConnected">Procesar</button>
          <button type="button" class="bg-slate-700 text-white p-2.5 rounded hover:bg-slate-600">P</button>
        </div>
      </div>
    </div>

    <!-- Second column (largest) -->
    <div class="overflow-y-auto prose prose-invert prose-h1:mb-4 prose-h2:mt-3 prose-h2:mb-2 max-w-none">
      <vue-markdown :source="mainContent" class="mr-4" />
    </div>

    <!-- Third column -->
    <div class="flex flex-col gap-2">
      <div ref="chatBox" class="h-full w-full p-2.5 bg-slate-700 border border-gray-600 rounded overflow-y-auto">
        <div v-for="(message, index) in ragChat" :key="index" :class="{
          'user-message text-right': message.src === 'USER',
          'nia-message text-left': message.src === 'NIA'
        }">
          <p :class="message.src === 'USER' ? 'bg-blue-500' : 'bg-green-900'" class="inline-block p-2 rounded-lg mb-4">
            {{ message.answer }}
          </p>
        </div>
      </div>
      <input type="text" v-model="ragQuestion"
        class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 ">
      <div class="flex justify-between gap-2">
        <button class="bg-slate-700 p-2.5 rounded hover:bg-slate-600" @click="showContext">R</button>
        <button class="bg-slate-700 p-2.5 rounded enabled:hover:bg-slate-600 flex-1 disabled:opacity-50"
          @click="ragQuery" :disabled="queryRagBtnDisabled">Enviar</button>
      </div>
    </div>
  </div>
</template>
