/**
 * 全局应用状态
 */
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { isDark, toggleTheme as toggle } from '@/composables/useTheme'

export const useAppStore = defineStore('app', () => {
  /** 移动端编辑器是否可见 */
  const mobileEditorVisible = ref(false)

  /** 侧边栏折叠（窄屏） */
  const sidebarCollapsed = ref(false)

  function toggleTheme(): void {
    toggle()
  }

  function showMobileEditor(): void {
    if (window.innerWidth > 600) return
    if (!mobileEditorVisible.value) mobileEditorVisible.value = true
  }

  function hideMobileEditor(): void {
    if (mobileEditorVisible.value) mobileEditorVisible.value = false
  }

  return { isDark, mobileEditorVisible, sidebarCollapsed, toggleTheme, showMobileEditor, hideMobileEditor }
})
