import { ref, onMounted, onUnmounted } from 'vue';

export function useWebSocket(url: string) {
  const socket = ref<WebSocket | null>(null);
  const isConnected = ref(false);

  const connectWebSocket = () => {
    console.log('Attempting to connect to WebSocket...');
    socket.value = new WebSocket(url);

    socket.value.onopen = () => {
      isConnected.value = true;
      console.log(new Date(), 'WebSocket connected');
    };

    socket.value.onmessage = (event) => {
      try {
        const jsonMessage = JSON.parse(event.data);
        if (onMessageCallback.value) {
          onMessageCallback.value(jsonMessage);
        }
      } catch (error) {
        console.error('Error parsing incoming message:', error);
      }
    };

    socket.value.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    socket.value.onclose = (event) => {
      console.log(new Date(), 'WebSocket connection closed', event.reason);
      isConnected.value = false;
      // Attempt to reconnect after 3 seconds
      setTimeout(connectWebSocket, 3000);
    };
  };

  const sendMessage = (msg: any): void => {
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

  const onMessageCallback = ref<((message: any) => void) | null>(null);

  const onMessage = (callback: (message: any) => void) => {
    onMessageCallback.value = callback;
  };

  onMounted(() => {
    connectWebSocket();
  });

  onUnmounted(() => {
    if (socket.value) {
      console.log('Closing WebSocket connection');
      socket.value.close();
    }
  });

  return {
    isConnected,
    sendMessage,
    onMessage
  };
}