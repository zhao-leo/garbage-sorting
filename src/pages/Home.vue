<template>
  <div class="video-container" @click="navigateToNewPage">>
    <!-- <video ref="videoPlayer" autoplay loop class="background-video">
      <source src="@/assets/1.mp4" type="video/mp4">
      Your browser does not support the video tag.
    </video> -->
    <img src="@/assets/1.gif" alt="Background" class="background-video" />
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { onMounted, onUnmounted } from 'vue';

export default {
  setup() {
    const router = useRouter();

    const navigateToNewPage = () => {
      router.push({ name: 'Panel' });
    };

    const setupEventListener = async () => {
      while (1) {
        await invoke('capture_and_save'); // 捕获图像并发送到后端
        console.log('Image captured'); // 添加日志
        await new Promise(resolve => setTimeout(resolve, 500)); // 等待0.5秒

        // 获取预测结果并检查
        const [_x, _y, label_id,_num] = await invoke('predict_image').catch((error) => {
          console.error('预测过程出错:', error);
          return [null, null, null],null;
        });

        // 如果获得有效的预测结果，跳转到面板页面
        if (label_id !== null && label_id !== undefined) {
          console.log('检测到物体，跳转到面板页面');
          navigateToNewPage();
          break; // 退出循环
        }
      }
    }
    onMounted(() => {
      invoke('initialize_model'); // 初始化模型
      console.log('Component mounted');
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
  cursor: pointer;
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