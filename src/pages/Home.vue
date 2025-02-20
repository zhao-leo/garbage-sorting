<template>
  <div class="video-container">
    <video ref="videoPlayer" autoplay loop class="background-video">
      <source src="@/assets/1.mp4" type="video/mp4">
      Your browser does not support the video tag.
    </video>
  </div>
</template>

<script>
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { onMounted, onUnmounted } from 'vue';

export default {
  setup() {
    const router = useRouter();
    console.log('Setup function called'); // 添加日志

    const navigateToNewPage = () => {
      console.log('Attempting to navigate...'); // 添加日志
      router.push({ name: 'NewPage' });
    };

    const setupEventListener = async () => {
      console.log('Setting up event listener...'); // 添加日志
      try {
        await listen('serial-data-received', (event) => {
          console.log('Event received:', event); // 添加日志
          const receivedData = event.payload;
          console.log('Received data in frontend:', receivedData);
          if (receivedData && receivedData.trim() !== '') {
            navigateToNewPage();
          }
        });
        console.log('Invoking listen_serial_port...'); // 添加日志
        await invoke('listen_serial_port');
        console.log('Successfully invoked listen_serial_port'); // 添加日志


      } catch (error) {
        console.error('Error in setupEventListener:', error); // 修改错误日志
      }
    };

    onMounted(() => {
      console.log('Component mounted'); // 添加日志
      setupEventListener();
    });

    onUnmounted(() => {
      console.log('Component unmounted'); // 添加日志
    });

    return {
      navigateToNewPage,
    };
  },
}
</script>

<style scoped>
.video-container {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.background-video {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  min-width: 90%;
  min-height: 90%;
  border-radius: 20px;
  overflow: hidden;
}
</style>