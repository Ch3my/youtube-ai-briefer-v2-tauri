<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';

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

const ragQuestion = ref('');
const ragContext = ref('');
const ragChat = ref<RagChatItem[]>([]);
const queryRagBtnDisabled = ref(false);
const chatBox = ref<HTMLElement | null>(null);

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
    ragContext.value += `Source: ${a.source}\n`;
    ragContext.value += `${a.content}\n\n`;
  }
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
    <input type="text" v-model="ragQuestion"
      class="border rounded block p-2.5 bg-slate-700 border-gray-600 placeholder-gray-400 ">
    <div class="flex justify-between gap-2">
      <button class="bg-slate-700 p-2.5 rounded hover:bg-slate-600" @click="showContext">R</button>
      <button class="bg-slate-700 p-2.5 rounded enabled:hover:bg-slate-600 flex-1 disabled:opacity-50"
        @click="ragQuery" :disabled="queryRagBtnDisabled || !props.isConnected">Enviar</button>
    </div>
  </div>
</template>