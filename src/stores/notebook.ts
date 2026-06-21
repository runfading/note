/**
 * 笔记本状态管理
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as notebookApi from '@/api/modules/notebook'
import type { NotebookInfo } from '@/types/notebook'

export const useNotebookStore = defineStore('notebook', () => {
  const notebooks = ref<NotebookInfo[]>([])
  const activeNotebookId = ref<number | null>(null)
  const loading = ref(false)

  /** 当前笔记本名称 */
  const activeNotebookName = computed(() => {
    if (activeNotebookId.value === null) return '全部笔记'
    const nb = notebooks.value.find(n => n.id === activeNotebookId.value)
    return nb?.name ?? '笔记'
  })

  /** 加载笔记本列表 */
  async function fetchNotebooks(): Promise<void> {
    loading.value = true
    try {
      // 一次性加载所有笔记本（page_size 设大一点）
      const result = await notebookApi.pageNotebooks({ page_num: 1, page_size: 100 })
      notebooks.value = result.list
    } catch (e) {
      console.error('加载笔记本失败:', e)
    } finally {
      loading.value = false
    }
  }

  /** 创建笔记本 */
  async function createNotebook(name: string): Promise<number | null> {
    try {
      const id = await notebookApi.createNotebook({ name })
      await fetchNotebooks()
      return id
    } catch (e) {
      console.error('创建笔记本失败:', e)
      return null
    }
  }

  /** 删除笔记本 */
  async function removeNotebook(id: number): Promise<boolean> {
    try {
      await notebookApi.removeNotebook(id)
      if (activeNotebookId.value === id) {
        activeNotebookId.value = null
      }
      await fetchNotebooks()
      return true
    } catch (e) {
      console.error('删除笔记本失败:', e)
      return false
    }
  }

  /** 设置当前笔记本 */
  function setActiveNotebook(id: number | null): void {
    activeNotebookId.value = id
  }

  return {
    notebooks,
    activeNotebookId,
    activeNotebookName,
    loading,
    fetchNotebooks,
    createNotebook,
    removeNotebook,
    setActiveNotebook,
  }
})
