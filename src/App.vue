<template>
  <router-view />
  <custom-dialog 
    v-if="showNotification"
    :title="notificationTitle"
    :message="notificationMessage"
    :duration="5000"
    @close="showNotification = false"
  />
</template>

<script>
import { onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import CustomDialog from '@/pages/CustomDialog.vue';
import { invoke } from '@tauri-apps/api/core';

export default {
  components: {
    CustomDialog
  },
  setup() {
    let unlisten = null;
    const showNotification = ref(false);
    const notificationTitle = ref('');
    const notificationMessage = ref('');
    invoke('setup_gpio_monitoring', { pin: 6 })
      .then(() => {
        console.log('GPIO monitoring setup successfully');
      })
      .catch((error) => {
        console.error('Error setting up GPIO monitoring:', error);
      });

      invoke('setup_gpio_monitoring', { pin: 13 })
      .then(() => {
        console.log('GPIO monitoring setup successfully');
      })
      .catch((error) => {
        console.error('Error setting up GPIO monitoring:', error);
      });

      invoke('setup_gpio_monitoring', { pin: 19 })
      .then(() => {
        console.log('GPIO monitoring setup successfully');
      })
      .catch((error) => {
        console.error('Error setting up GPIO monitoring:', error);
      });

      invoke('setup_gpio_monitoring', { pin: 26 })
      .then(() => {
        console.log('GPIO monitoring setup successfully');
      })
      .catch((error) => {
        console.error('Error setting up GPIO monitoring:', error);
      });
    onMounted(async () => {
      // 监听GPIO下降沿事件
      unlisten = await listen('gpio-falling-edge', (event) => {
        // event.payload 包含 pin 角信息
        const pinNumber = event.payload;
        
        // 显示自定义弹窗
        notificationTitle.value = 'GPIO信号通知';
        notificationMessage.value = `检测到GPIO引脚 ${pinNumber} 下降沿信号`;
        showNotification.value = true;
      });
    });

    onUnmounted(() => {
      if (unlisten) unlisten();
    });

    return {
      showNotification,
      notificationTitle,
      notificationMessage
    };
  }
}
</script>

<style>
@font-face {
  font-family: 'CustomFont';
  src: local('HarmonyOS Sans Regular'),
    url('@/assets/HarmonyOS_Sans_Regular.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
  font-display: swap;
}

html,
body {
  overflow: hidden;
  margin: 0;
  padding: 0;
  font-family: 'CustomFont', system-ui, -apple-system, sans-serif;
  color: #000000;
  font-size: 16px;
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}

* {
  box-sizing: border-box;
}
</style>