/**
 * 主题切换 composable
 * 控制 data-theme 属性（dark / 无属性 = light）
 */
import { ref, watchEffect } from 'vue'
import { getTheme, setTheme } from '@/utils/storage'

/** 当前是否为暗色模式 */
export const isDark = ref(getTheme() === 'dark')

/** 初始化主题（在应用启动时调用一次） */
export function initTheme(): void {
  if (isDark.value) {
    document.documentElement.setAttribute('data-theme', 'dark')
  } else {
    document.documentElement.removeAttribute('data-theme')
  }
}

/** 切换主题 */
export function toggleTheme(): void {
  isDark.value = !isDark.value
}

// 监听 isDark 变化，同步到 DOM 和 localStorage
watchEffect(() => {
  if (isDark.value) {
    document.documentElement.setAttribute('data-theme', 'dark')
    setTheme('dark')
  } else {
    document.documentElement.removeAttribute('data-theme')
    setTheme('light')
  }
})
