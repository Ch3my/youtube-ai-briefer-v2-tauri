<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import Modal from './Modal.vue';

type Context = {
  source: string;
  content: string;
};

type RagChatItem = {
  answer: string;
  context?: Context[];
  src: string
};

const props = defineProps<{
  isConnected: boolean;
}>();

const emit = defineEmits(['sendMessage']);

function handleKeyUp(event: KeyboardEvent) {
  if (event.key === 'Enter') {
    ragQuery();
  }
}
const ragQuestion = ref('');
const ragContext = ref('');
const ragChat = ref<RagChatItem[]>([]);
const queryRagBtnDisabled = ref(false);
const chatBox = ref<HTMLElement | null>(null);

const isModalOpen = ref(false);


watch(ragChat, async () => {
  await nextTick();
  if (chatBox.value != null) {
    chatBox.value.scrollTop = chatBox.value.scrollHeight;
  }
}, { deep: true });

function ragQuery() {
  queryRagBtnDisabled.value = true;
  emit('sendMessage', {
    query: ragQuestion.value,
    action: "query"
  });
  ragChat.value.push({
    answer: ragQuestion.value,
    src: "USER"
  });
  ragQuestion.value = "";
}

function showContext() {
  const lastNiaAnswer = ragChat.value
    .filter(item => item.src === "NIA")
    .pop();

  if (!lastNiaAnswer || !lastNiaAnswer.context) {
    return;
  }
  ragContext.value = "";
  for (const a of lastNiaAnswer.context) {
    ragContext.value += `<p class="font-bold">Source: ${a.source}</p>`;
    ragContext.value += `<p class="mb-4">${a.content}</p>`;
  }
  isModalOpen.value = true
}

function handleBackendMessage(jsonMessage: any) {
  if (jsonMessage.action == "ragAnswer") {
    queryRagBtnDisabled.value = false;
    ragChat.value.push({
      answer: jsonMessage.answer,
      context: jsonMessage.context,
      src: "NIA"
    });
  }
}

defineExpose({ handleBackendMessage });
</script>

<template>
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
    <input type="text" v-model="ragQuestion" @keyup="handleKeyUp"
      class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 ">
    <div class="flex justify-between gap-2">
      <button class="bg-slate-700 p-2.5 rounded hover:bg-slate-600" @click="showContext">
        <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-vocabulary" width="22" height="22"
          viewBox="0 0 24 24" stroke-width="1.5" stroke="white" fill="none" stroke-linecap="round"
          stroke-linejoin="round">
          <path stroke="none" d="M0 0h24v24H0z" fill="none" />
          <path
            d="M10 19h-6a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1h6a2 2 0 0 1 2 2a2 2 0 0 1 2 -2h6a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-6a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2z" />
          <path d="M12 5v16" />
          <path d="M7 7h1" />
          <path d="M7 11h1" />
          <path d="M16 7h1" />
          <path d="M16 11h1" />
          <path d="M16 15h1" />
        </svg>
      </button>
      <button class="bg-slate-700 p-2.5 rounded enabled:hover:bg-slate-600 flex-1 disabled:opacity-50" @click="ragQuery"
        :disabled="queryRagBtnDisabled || !props.isConnected">Enviar</button>
    </div>
    <Modal v-model="isModalOpen" title="Contexto">
      <div v-html="ragContext">
      </div>
    </Modal>
  </div>
</template>