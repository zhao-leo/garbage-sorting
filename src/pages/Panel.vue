<template>
  <div class="panel">
    <h1>垃圾分类统计面板</h1>

    <!-- 垃圾统计数据 -->
    <div class="statistics">
      <div class="stat-item">
        <h3>可回收物</h3>
        <p>{{ trashCounts.recyclable }}</p>
      </div>
      <div class="stat-item">
        <h3>有害垃圾</h3>
        <p>{{ trashCounts.harmful }}</p>
      </div>
      <div class="stat-item">
        <h3>厨余垃圾</h3>
        <p>{{ trashCounts.kitchen }}</p>
      </div>
      <div class="stat-item">
        <h3>其他垃圾</h3>
        <p>{{ trashCounts.other }}</p>
      </div>
    </div>

    <!-- 最近投放记录 -->
    <div class="history">
      <h2>最近投放记录</h2>
      <ul>
        <li v-for="(item, index) in trashHistory" :key="index">
          {{ item.type }} - {{ item.time }}
        </li>
      </ul>
    </div>

    <!-- <router-link to="/home" class="home-link">返回首页</router-link> -->
  </div>
</template>

<script setup>
import { ref, onMounted,onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'

const router = useRouter()
const trashCounts = ref({
  recyclable: 0,
  harmful: 0,
  kitchen: 0,
  other: 0
})

const trashHistory = ref([])
const MAX_HISTORY = 20
let timer = null
let countdownTimer = null

// 添加新的垃圾记录
const addTrash = (type) => {
  trashCounts.value[type]++
  trashHistory.value.unshift({
    type: type,
    time: new Date().toLocaleTimeString()
  })
  if (trashHistory.value.length > MAX_HISTORY) {
    trashHistory.value.pop()
  }
}

// 延迟函数
const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms))

// 开始5分钟倒计时
const startCountdown = () => {
  let countdown = 5 * 60 // 5分钟
  countdownTimer = setInterval(() => {
    countdown--
    if (countdown <= 0) {
      clearInterval(countdownTimer)
      router.push('/home')
    }
  }, 1000)
}

// 循环检测功能
const startDetection = async () => {
  while (true) {
    try {
      // 等待1秒
      await delay(1000)

      // 捕获并保存图片
      const captureResult = await invoke('capture_and_save')
      if (!captureResult) {
        console.error('图片捕获失败')
        await delay(1000)
        continue
      }

      // 进行预测
      const [class_id, confidence, label] = await invoke('predict_image')
      console.log('预测结果:', { class_id, confidence, label })

      // 如果预测结果为空（假设 class_id < 0 表示无效结果）
      if (class_id < 0) {
        clearInterval(timer)
        startCountdown()
        break
      }

      // 根据预测结果更新统计
      const typeMap = {
        0: 'recyclable',
        1: 'harmful',
        2: 'kitchen',
        3: 'other'
      }
      const type = typeMap[class_id]
      if (type) {
        addTrash(type)
      }

    } catch (error) {
      console.error('检测过程出错:', error)
    }
  }
}

onMounted(async () => {
  // 开始检测循环
  startDetection()
})

// 组件卸载时清理定时器
onUnmounted(() => {
  if (timer) clearInterval(timer)
  if (countdownTimer) clearInterval(countdownTimer)
})
</script>

<style scoped>
.panel {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

h1 {
  color: #000000;
  text-align: center;
  margin-bottom: 30px;
}

.statistics {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
  margin: 20px 0;
}

.stat-item {
  padding: 15px;
  border-radius: 8px;
  background-color: #f5f5f5;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease;
}

.stat-item:hover {
  transform: translateY(-2px);
}

.stat-item h3 {
  margin: 0;
  color: #000000;
  font-size: 18px;
}

.stat-item p {
  font-size: 24px;
  font-weight: bold;
  margin: 10px 0 0;
  color: #000000;
}

.history {
  margin-top: 30px;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
}

.history h2 {
  color: #000000;
  margin-top: 0;
  margin-bottom: 15px;
}

.history ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.history li {
  padding: 12px;
  border-bottom: 1px solid #e0e0e0;
  color: #000000;
  background-color: white;
  margin-bottom: 8px;
  border-radius: 4px;
}

.history li:last-child {
  border-bottom: none;
  margin-bottom: 0;
}

.home-link {
  display: inline-block;
  margin-top: 20px;
  padding: 10px 20px;
  background-color: #f5f5f5;
  color: #000000;
  text-decoration: none;
  border-radius: 4px;
  transition: all 0.3s ease;
  border: 1px solid #e0e0e0;
}

.home-link:hover {
  background-color: #e0e0e0;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
</style>