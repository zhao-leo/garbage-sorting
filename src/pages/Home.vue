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
  import { onMounted } from 'vue';
  
  export default {
    setup() {
      // const router = useRouter();
  
      const navigateToNewPage = () => {
        this.$router.push({ name: 'NewPage' });
      };
  
      const setupEventListener = async () => {
        await invoke('listen_serial_port');
        await listen('serial-data-received', (event) => {
          const receivedData = event.payload;
          if (receivedData && receivedData.trim() !== '') {
            // Handle the event and navigate to the new page
            navigateToNewPage();
          }
        });
      };
  
      onMounted(() => {
        setupEventListener();
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