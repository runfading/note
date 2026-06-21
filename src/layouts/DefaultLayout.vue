<script setup lang="ts">
/**
 * 默认三栏布局 —— 侧边栏 | 笔记列表 | 编辑器
 */
import { onMounted, onUnmounted, watch } from 'vue'
import { useNoteStore } from '@/stores/note'
import { useNotebookStore } from '@/stores/notebook'
import { useAppStore } from '@/stores/app'
import { useToast } from '@/composables/useDialog'
import AppSidebar from '@/components/AppSidebar.vue'
import NoteListPanel from '@/components/NoteListPanel.vue'
import EditorPanel from '@/components/EditorPanel.vue'

const noteStore = useNoteStore()
const notebookStore = useNotebookStore()
const appStore = useAppStore()
const { showToast } = useToast()

onMounted(async () => {
  await notebookStore.fetchNotebooks()
  await noteStore.fetchNotesByNotebook(null)
  window.addEventListener('keydown', onKeydown)
  setupResizeListener()
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
  cleanupResizeListener()
})

// 监听当前活跃笔记本变化，重新加载笔记
watch(
  () => notebookStore.activeNotebookId,
  async (id) => {
    await noteStore.fetchNotesByNotebook(id)
  },
)

// Ctrl+S 手动保存
function onKeydown(e: KeyboardEvent): void {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    // 保存当前笔记
    if (noteStore.activeNoteId) {
      noteStore.updateNote(noteStore.activeNoteId, {
        title: noteStore.activeNote?.title,
        content: noteStore.activeNote?.content,
      })
      showToast('✅ 已保存')
    }
  }
  // Ctrl+N 新笔记
  if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
    e.preventDefault()
    // 触发创建
  }
  // Esc 关闭移动端编辑器
  if (e.key === 'Escape' && window.innerWidth <= 600) {
    appStore.hideMobileEditor()
  }
}

// 使用 matchMedia 替代 resize 事件，避免频繁触发导致 DOM 冲突
let mediaQuery: MediaQueryList | null = null

function setupResizeListener(): void {
  mediaQuery = window.matchMedia('(min-width: 601px)')
  mediaQuery.addEventListener('change', (e) => {
    if (e.matches) {
      appStore.hideMobileEditor()
    }
  })
}

function cleanupResizeListener(): void {
  mediaQuery?.removeEventListener('change', () => {})
  mediaQuery = null
}
</script>

<template>
  <div class="app-layout">
    <AppSidebar />
    <NoteListPanel />
    <EditorPanel />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
