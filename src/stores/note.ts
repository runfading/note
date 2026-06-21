/**
 * 笔记状态管理
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as noteApi from '@/api/modules/note'
import * as noteTagApi from '@/api/modules/note-tag'
import type { NoteInfo } from '@/types/note'

export const useNoteStore = defineStore('note', () => {
  const notes = ref<NoteInfo[]>([])
  const activeNoteId = ref<number | null>(null)
  const searchQuery = ref('')
  const loading = ref(false)
  const total = ref(0)
  const currentPage = ref(1)
  const pageSize = ref(50)

  /** 当前激活的笔记 */
  const activeNote = computed(() => {
    if (activeNoteId.value === null) return null
    return notes.value.find(n => n.id === activeNoteId.value) ?? null
  })

  /** 按笔记本加载笔记（清除搜索），加载后自动选中第一篇 */
  async function fetchNotesByNotebook(notebookId?: number | null): Promise<void> {
    searchQuery.value = ''
    loading.value = true
    try {
      const result = await noteApi.pageNotes({
        notebook_id: notebookId ?? undefined,
        page_num: currentPage.value,
        page_size: pageSize.value,
      })
      notes.value = result.list
      total.value = result.total
      // 自动选中第一篇笔记（仅在无选中时，创建笔记后已有选中不会被覆盖）
      if (result.list.length > 0 && activeNoteId.value === null) {
        activeNoteId.value = result.list[0].id
      } else if (result.list.length === 0) {
        activeNoteId.value = null
      }
    } catch (e) {
      console.error('加载笔记失败:', e)
    } finally {
      loading.value = false
    }
  }

  /** 搜索笔记 */
  async function searchNotes(keyword: string): Promise<void> {
    searchQuery.value = keyword
    currentPage.value = 1
    if (!keyword.trim()) {
      // 空搜索 → 恢复按笔记本加载
      await fetchNotesByNotebook(undefined)
      return
    }
    loading.value = true
    try {
      const result = await noteApi.searchNotes({
        keyword: keyword.trim(),
        page_num: currentPage.value,
        page_size: pageSize.value,
      })
      notes.value = result.list
      total.value = result.total
    } catch (e) {
      console.error('搜索笔记失败:', e)
    } finally {
      loading.value = false
    }
  }

  /** 获取笔记详情 */
  async function fetchNoteDetail(id: number): Promise<NoteInfo | null> {
    try {
      return await noteApi.noteDetail(id)
    } catch (e) {
      console.error('获取笔记详情失败:', e)
      return null
    }
  }

  /** 创建笔记 */
  async function createNote(notebookId: number): Promise<number | null> {
    try {
      const id = await noteApi.createNote({
        notebook_id: notebookId,
        title: '',
        content: '',
      })
      return id
    } catch (e) {
      console.error('创建笔记失败:', e)
      return null
    }
  }

  /** 更新笔记 */
  async function updateNote(
    noteId: number,
    data: { title?: string; content?: string; notebook_id?: number },
  ): Promise<boolean> {
    try {
      await noteApi.updateNote({
        note_id: noteId,
        ...data,
      })
      return true
    } catch (e) {
      console.error('更新笔记失败:', e)
      return false
    }
  }

  /** 删除笔记 */
  async function removeNote(id: number): Promise<boolean> {
    try {
      await noteApi.removeNote(id)
      if (activeNoteId.value === id) {
        activeNoteId.value = null
      }
      return true
    } catch (e) {
      console.error('删除笔记失败:', e)
      return false
    }
  }

  /** 给笔记添加标签 */
  async function addTag(noteId: number, tagName: string): Promise<boolean> {
    try {
      await noteTagApi.addNoteTag(noteId, tagName)
      return true
    } catch (e) {
      console.error('添加标签失败:', e)
      return false
    }
  }

  /** 移除笔记标签 */
  async function removeTag(noteId: number, tagName: string): Promise<boolean> {
    try {
      await noteTagApi.deleteNoteTag(noteId, tagName)
      return true
    } catch (e) {
      console.error('移除标签失败:', e)
      return false
    }
  }

  /** 选中笔记 */
  function selectNote(id: number | null): void {
    activeNoteId.value = id
  }

  /** 更新本地笔记缓存（编辑器内容变更时） */
  function updateLocalNote(noteId: number, data: { title?: string; content?: string }): void {
    const note = notes.value.find(n => n.id === noteId)
    if (note) {
      if (data.title !== undefined) note.title = data.title
      if (data.content !== undefined) note.content = data.content
    }
  }

  return {
    notes,
    activeNoteId,
    activeNote,
    searchQuery,
    loading,
    total,
    currentPage,
    fetchNotesByNotebook,
    searchNotes,
    fetchNoteDetail,
    createNote,
    updateNote,
    removeNote,
    addTag,
    removeTag,
    selectNote,
    updateLocalNote,
  }
})
