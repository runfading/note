<script setup lang="ts">
/**
 * 笔记列表面板 —— 搜索 + 笔记列表 + 新建按钮
 */
import { ref, watch } from 'vue'
import { useNoteStore } from '@/stores/note'
import { useNotebookStore } from '@/stores/notebook'
import { useTagStore } from '@/stores/tag'
import { useAppStore } from '@/stores/app'
import { useToast } from '@/composables/useDialog'
import SearchBar from './SearchBar.vue'
import NoteItem from './NoteItem.vue'

const noteStore = useNoteStore()
const notebookStore = useNotebookStore()
const tagStore = useTagStore()
const appStore = useAppStore()
const { showToast } = useToast()

const searchText = ref('')
let searchTimer: ReturnType<typeof setTimeout> | null = null

/** 标题文本 */
const listTitle = ref('全部笔记')

watch(
  () => [notebookStore.activeNotebookId, tagStore.activeTagFilter, noteStore.notes],
  () => {
    if (tagStore.activeTagFilter) {
      listTitle.value = `🏷️ ${tagStore.activeTagFilter}`
    } else if (notebookStore.activeNotebookId !== null) {
      listTitle.value = notebookStore.activeNotebookName
    } else {
      listTitle.value = '全部笔记'
    }
  },
  { immediate: true },
)

/** 搜索输入 */
function onSearch(val: string): void {
  searchText.value = val
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(() => {
    noteStore.searchNotes(val)
  }, 300)
}

/** 选择笔记 */
function selectNote(noteId: number): void {
  noteStore.selectNote(noteId)
  appStore.showMobileEditor()
}

/** 新建笔记 */
async function createNote(): Promise<void> {
  const nbId = notebookStore.activeNotebookId ?? notebookStore.notebooks[0]?.id ?? 0
  if (!nbId) {
    showToast('请先创建笔记本')
    return
  }
  const id = await noteStore.createNote(nbId)
  if (id) {
    noteStore.selectNote(id)
    // 刷新列表
    await noteStore.fetchNotesByNotebook(notebookStore.activeNotebookId)
    showToast('新笔记已创建')
  }
}
</script>

<template>
  <div class="note-list-panel">
    <div class="list-header">
      <div class="list-header-top">
        <button class="back-btn-mobile" @click="appStore.hideMobileEditor()">← 返回</button>
        <span class="list-title">{{ listTitle }}</span>
        <button class="new-note-btn" title="新建笔记" @click="createNote">+</button>
      </div>
      <SearchBar :model-value="searchText" @update:model-value="onSearch" />
    </div>

    <div class="note-items">
      <div v-if="noteStore.notes.length === 0" class="empty-notes">
        <span class="empty-icon">📭</span>
        <span>{{ searchText ? '没有匹配的笔记' : '这里还没有笔记' }}</span>
        <span class="hint" v-if="!searchText">点击右上角 + 创建</span>
      </div>
      <NoteItem
        v-for="note in noteStore.notes"
        :key="note.id"
        :note="note"
        :active="noteStore.activeNoteId === note.id"
        @select="selectNote"
      />
    </div>
  </div>
</template>

<style scoped>
.note-list-panel {
  width: var(--list-width);
  min-width: var(--list-width);
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  height: 100vh;
  transition: all var(--transition);
}
.list-header {
  padding: 14px 16px;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex-shrink: 0;
}
.list-header-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.list-title {
  font-weight: 700;
  font-size: 1rem;
  color: var(--text-primary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.new-note-btn {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  border: none;
  background: var(--accent);
  color: #fff;
  cursor: pointer;
  font-size: 1.3rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition);
  flex-shrink: 0;
  line-height: 1;
}
.new-note-btn:hover {
  background: var(--accent-hover);
  transform: scale(1.05);
  box-shadow: var(--shadow-md);
}
.note-items {
  flex: 1;
  overflow-y: auto;
  padding: 6px 8px;
}
.empty-notes {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-tertiary);
  font-size: 0.85rem;
}
.empty-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 12px;
  opacity: 0.5;
}
.hint {
  font-size: 0.7rem;
  display: block;
  margin-top: 4px;
}
.back-btn-mobile {
  display: none;
  align-items: center;
  gap: 4px;
  font-size: 0.85rem;
  color: var(--accent);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  border: none;
  background: transparent;
}
</style>
