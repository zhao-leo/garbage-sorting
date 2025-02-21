import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/Home.vue'
import NewPage from './pages/NewPage.vue'
import Panel from './pages/Panel.vue'

const routes = [
  {
    path: '/',
    redirect: '/panel'  // 添加重定向
  },
  {
    path: '/home',
    name: 'Home',
    component: Home
  },
  {
    path: '/new-page',
    name: 'NewPage',
    component: NewPage
  },
  {
    path: '/panel',
    name: 'Panel',
    component: Panel
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router