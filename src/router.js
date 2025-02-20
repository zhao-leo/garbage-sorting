import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/Home.vue'
import NewPage from './pages/NewPage.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/new-page',
    name: 'NewPage',
    component: NewPage
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router