<script setup lang="ts">
import { ref } from 'vue';
import BackendFeedback from "./components/BackendFeedback.vue";
import BackendStatus from "./components/BackendStatus.vue";
import SettingsBtn from "./components/SettingsBtn.vue";
import VideoInfo from "./components/VideoInfo.vue";
import RagChat from './components/RagChat.vue';
import VueMarkdown from 'vue-markdown-render'
import ClipboardBtn from "./components/ClipboardBtn.vue";
import CopyButtons from "./components/CopyBtns.vue";
// import { invoke } from "@tauri-apps/api/tauri";

// @ts-ignore
import { getYoutubeMediaInfo } from "./youtube-helper"

import { useWebSocket } from './composables/useWebSocket';
const { isConnected, sendMessage, onMessage } = useWebSocket('ws://localhost:12345');

const videoUrl = ref('');
const mainContent = ref('');
const videoTitle = ref('');
const videoImg = ref('');
const processBtnDisabled = ref(false);
const feedbackType = ref('')
const feedbackText = ref('')
const whisperConfirmed = ref(false)
const originalNotes = ref<string[]>([])
const originalTranscript = ref('')
const chatComponent = ref<InstanceType<typeof RagChat> | null>(null);

const handleClipboardContent = (content: string) => {
  videoUrl.value = content;
};
function handleKeyUp(event: KeyboardEvent) {
  if (event.key === 'Enter') {
    buildVideoData();
  }
}

function handleCopied(msg: string) {
  feedbackType.value = "info"
  feedbackText.value = msg
}

function buildVideoData() {
  if (videoUrl.value == "") {
    feedbackType.value = "alert"
    feedbackText.value = "Debes ingresar una Url"
    return
  }
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
  if (jsonMessage.action == "noteSection") {
    originalNotes.value.push(jsonMessage.note)
  }
  if (jsonMessage.action == "transcript") {
    originalTranscript.value = jsonMessage.transcript
  }
}
// Set up the message handler
onMessage(handleBackendMessage)
</script>

<template>
  <div class="grid h-screen p-4 gap-4 grid-rows-1" style="grid-template-columns: 1fr 4fr 2fr;">
    <!-- First column -->
    <div class="flex gap-2 flex-col justify-between">
      <div class="flex gap-4 flex-col">
        <div class="flex justify-between">
          <BackendStatus :is-connected="isConnected" />
          <SettingsBtn />
        </div>
        <CopyButtons :original-transcript="originalTranscript" :original-notes="originalNotes"
          :main-content="mainContent" @copied="handleCopied" />
      </div>
      <VideoInfo :video-title="videoTitle" :video-img="videoImg" />
      <div class="flex flex-col gap-2">
        <BackendFeedback :type="feedbackType" :txt="feedbackText" />
        <input v-model="videoUrl" type="text" @keyup="handleKeyUp"
          class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 "
          placeholder="Youtube URL" />
        <div class="flex gap-2">
          <button class="bg-slate-700  p-2.5 rounded enabled:hover:bg-slate-600 flex-1 disabled:opacity-50"
            @click="buildVideoData" :disabled="processBtnDisabled || !isConnected">Procesar</button>
          <ClipboardBtn @clipboard-content="handleClipboardContent" />
        </div>
      </div>
    </div>

    <!-- Second column (largest) -->
    <div
      class="overflow-y-auto prose prose-invert prose-h1:mb-4 prose-h2:mt-3 prose-h2:mb-2 prose-h3:mt-3 prose-h3:mb-2 max-w-none">
      <vue-markdown :source="mainContent" class="mr-4" />
    </div>

    <!-- Third column -->
    <RagChat ref="chatComponent" :is-connected="isConnected" @send-message="sendMessage" />
  </div>
</template>
