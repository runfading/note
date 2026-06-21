import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './assets/styles/global.css'

// 禁用浏览器原生右键菜单（Tauri 桌面应用）
document.addEventListener('contextmenu', (e) => e.preventDefault())

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
