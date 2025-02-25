<template>
  <div class="panel">
    <h1>垃圾分类统计面板</h1>

    <!-- 垃圾统计数据 -->
    <div class="statistics">
      <div class="stat-item">
        <h3>可回收物</h3>
        <p>
          {{ trashCounts.recyclable }}
        <div v-if="showProgress.recyclable" class="progress-bar"></div>
        </p>
      </div>
      <div class="stat-item">
        <h3>有害垃圾</h3>
        <p>
          {{ trashCounts.harmful }}
        <div v-if="showProgress.harmful" class="progress-bar"></div>
        </p>
      </div>
      <div class="stat-item">
        <h3>厨余垃圾</h3>
        <p>
          {{ trashCounts.kitchen }}
        <div v-if="showProgress.kitchen" class="progress-bar"></div>
        </p>
      </div>
      <div class="stat-item">
        <h3>其他垃圾</h3>
        <p>
          {{ trashCounts.other }}
        <div v-if="showProgress.other" class="progress-bar"></div>
        </p>
      </div>
    </div>

    <!-- 最近投放记录 -->
    <div class="history">
      <h2>最近投放记录</h2>
      <ul>
        <li v-for="(item, index) in trashHistory" :key="index">
          {{ trashHistory.length - index }}. {{ typeToChineseMap[item.type] }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
// import { useRouter } from 'vue-router'

// const router = useRouter()
const trashCounts = ref({
  recyclable: 0,
  harmful: 0,
  kitchen: 0,
  other: 0
})

const showProgress = ref({
  recyclable: false,
  harmful: false,
  kitchen: false,
  other: false
})

const typeToChineseMap = {
  'recyclable': '可回收物',
  'harmful': '有害垃圾',
  'kitchen': '厨余垃圾',
  'other': '其他垃圾'
}


const trashHistory = ref([])
const MAX_HISTORY = 20

// 添加新的垃圾记录
const addTrash = (type) => {
  trashCounts.value[type]++
  showProgress.value[type] = true
  setTimeout(() => {
    showProgress.value[type] = false
  }, 1000)

  trashHistory.value.unshift({
    type: type
  })
  if (trashHistory.value.length > MAX_HISTORY) {
    trashHistory.value.pop()
  }
}

// 延迟函数
const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms))

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
      const [x, y, label_id] = await invoke('predict_image').catch((error) => {
        console.error('预测过程出错:', error)
      })
      console.log('预测结果:', { x, y, label_id })

      // 根据预测结果更新统计
      const typeMap = {
        0: 'harmful',
        1: 'harmful',
        2: 'harmful',
        3: 'kitchen',
        4: 'kitchen',
        5: 'kitchen',
        6: 'kitchen',
        7: 'kitchen',
        8: 'other',
        9: 'other',
        10: 'other',
        11: 'recyclable',
        12: 'recyclable',
        13: 'recyclable',
        14: 'recyclable',
        15: 'recyclable',
      }
      const type = typeMap[label_id]
      if (type) {
        addTrash(type)
      }

    } catch (error) {
      console.error('检测过程出错:', error)
    }
  }
}

onMounted(async () => {
  await invoke('initialize_model').catch((error) => {
    console.error('模型初始化失败:', error)
  })
  startDetection()
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
  position: relative;
  font-size: 24px;
  font-weight: bold;
  margin: 10px 0 0;
  color: #000000;
  padding-bottom: 4px;
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
  display: flex;
  align-items: center;
}

.progress-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 10px;
  background-color: #4CAF50;
  animation: progress 1s linear;
}

@keyframes progress {
  0% {
    width: 0;
  }

  100% {
    width: 100%;
  }
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