<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import BackendFeedback from "./components/BackendFeedback.vue";
import BackendStatus from "./components/BackendStatus.vue";
import SettingsBtn from "./components/SettingsBtn.vue";
import VideoInfo from "./components/VideoInfo.vue";
// import { invoke } from "@tauri-apps/api/tauri";
import VueMarkdown from 'vue-markdown-render'

import { useWebSocket } from './composables/useWebSocket';
const { isConnected, sendMessage, onMessage } = useWebSocket('ws://localhost:12345');

import RagChat from './components/RagChat.vue';

// @ts-ignore
import { getYoutubeMediaInfo } from "./youtube-helper"

const videoUrl = ref('');
const mainContent = ref('');
const videoTitle = ref('');
const videoImg = ref('');
const processBtnDisabled = ref(false);
const feedbackType = ref('')
const feedbackText = ref('')

const whisperConfirmed = ref(false)

const chatComponent = ref<InstanceType<typeof RagChat> | null>(null);

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

// onMounted(() => {
// });

// onUnmounted(() => {
// });

async function fetchMetaYT(url: string = "https://www.youtube.com/watch?v=dQw4w9WgXcQ") {
  let result = await getYoutubeMediaInfo(url)
  if (!result) {
    console.log("Error al obtener datos del Video");
    return
  }
  videoTitle.value = result.title
  videoImg.value = result.thumbnailUrl
}

function handleBackendMessage(jsonMessage: any) {
  if (jsonMessage.action == "notasDetalladas") {
    feedbackType.value = "success"
    feedbackText.value = "Procesamiento completado"
    mainContent.value = jsonMessage.notasDetalladas
    processBtnDisabled.value = false
  }
  if (jsonMessage.action == "ragAnswer") {
    chatComponent.value?.handleBackendMessage(jsonMessage);
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
// Set up the message handler
onMessage(handleBackendMessage)
</script>

<template>
  <div class="grid h-screen p-4 gap-4 grid-rows-1" style="grid-template-columns: 1fr 4fr 2fr;">
    <!-- First column -->
    <div class="flex gap-2 flex-col justify-between">
      <div class="flex justify-between">
        <BackendStatus :is-connected="isConnected" />
        <!-- <button type="button"
          class="flex items-center text-white  focus:outline-none focus:ring-4  font-medium rounded-lg text-sm px-3 py-1.5 bg-slate-700 hover:bg-slate-600 focus:ring-gray-700 border-gray-700">
          <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-settings" width="24" height="24"
            viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
            stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path
              d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z" />
            <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" />
          </svg>
        </button> -->
        <SettingsBtn />
      </div>
      <VideoInfo :video-title="videoTitle" :video-img="videoImg" />
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
    <RagChat ref="chatComponent" :is-connected="isConnected" @send-message="sendMessage" />
  </div>
</template>
