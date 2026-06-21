<script setup lang="ts">
/**
 * 笔记本列表 —— 展示所有笔记本，支持选择和删除
 */
import {onMounted} from 'vue'
import {useNotebookStore} from '@/stores/notebook'
import {useNoteStore} from '@/stores/note'
import {useTagStore} from '@/stores/tag'
import {useToast, useDialog, useConfirmDialog} from '@/composables/useDialog'
import {escapeHtml} from '@/utils/format'

const notebookStore = useNotebookStore()
const noteStore = useNoteStore()
const tagStore = useTagStore()
const {showToast} = useToast()
const {showDialog} = useDialog()
const {showConfirm} = useConfirmDialog()

onMounted(() => {
  notebookStore.fetchNotebooks()
})

/** 选中笔记本 */
async function selectNotebook(id: number | null): Promise<void> {
  notebookStore.setActiveNotebook(id)
  tagStore.setTagFilter(null)
  await noteStore.fetchNotesByNotebook(id)
}

/** 删除笔记本 */
async function onDelete(nbId: number, nbName: string, e: Event): Promise<void> {
  e.stopPropagation()
  const confirmed = await showConfirm({
    title: '删除笔记本',
    message: `确定删除"${nbName}"？`,
  })
  if (confirmed) {
    await notebookStore.removeNotebook(nbId)
    await noteStore.fetchNotesByNotebook(null)
    showToast('笔记本已删除')
  }
}

/** 重命名笔记本 */
async function onRename(_nbId: number, nbName: string): Promise<void> {
  const newName = await showDialog({
    title: '重命名笔记本',
    defaultValue: nbName,
    confirmText: '保存',
  })
  if (newName) {
    // 通过 create + delete 实现重命名...但 backend 没有 rename API
    // 这里简化处理：提示用户（实际需要后端支持）
    showToast('重命名功能需要后端支持')
  }
}

/** 获取笔记本笔记数量 */
function noteCount(nbId: number): number {
  return noteStore.notes.filter(n => n.notebook_id === nbId).length
}
</script>

<template>
  <div class="notebooks-section">
    <span class="section-label">📚 笔记本</span>
    <ul class="notebook-list">
      <!-- 全部笔记 -->
      <li
          class="notebook-item"
          :class="{ active: notebookStore.activeNotebookId === null && !tagStore.activeTagFilter }"
          @click="selectNotebook(null)"
      >
        <span class="icon">📋</span>
        <span class="name">全部笔记</span>
        <span class="count">{{ noteStore.notes.length }}</span>
      </li>

      <!-- 各笔记本 -->
      <li
          v-for="nb in notebookStore.notebooks"
          :key="nb.id"
          class="notebook-item"
          :class="{ active: notebookStore.activeNotebookId === nb.id && !tagStore.activeTagFilter }"
          @click="selectNotebook(nb.id)"
          @dblclick="onRename(nb.id, nb.name)"
      >
        <span class="icon">📓</span>
        <span class="name">{{ escapeHtml(nb.name) }}</span>
        <span class="count">{{ noteCount(nb.id) }}</span>
        <button class="delete-btn" @click="onDelete(nb.id, nb.name, $event)">×</button>
      </li>
    </ul>

    <button class="add-notebook-btn" @click="async () => {
      const name = await showDialog({ title: '新建笔记本', confirmText: '创建' })
      if (name) {
        const id = await notebookStore.createNotebook(name)
        if (id) {
          notebookStore.setActiveNotebook(id)
          await noteStore.fetchNotesByNotebook(id)
          showToast('笔记本已创建')
        }
      }
    }">
      + 新建笔记本
    </button>
  </div>
</template>

<style scoped>
.notebooks-section {
  flex: 1;
  overflow-y: auto;
  padding: 12px 0;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.section-label {
  font-size: 0.7rem;
  text-transform: uppercase;
  letter-spacing: 1.2px;
  color: var(--text-tertiary);
  padding: 8px 20px 6px;
  font-weight: 600;
  user-select: none;
}

.notebook-list {
  list-style: none;
  flex: 1;
  overflow-y: auto;
  padding: 0 10px;
}

.notebook-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 9px 12px;
  border-radius: var(--radius);
  cursor: pointer;
  transition: all var(--transition);
  font-size: 0.9rem;
  color: var(--text-secondary);
  position: relative;
  user-select: none;
  margin-bottom: 1px;
}

.notebook-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.notebook-item.active {
  background: var(--bg-active);
  color: var(--accent);
  font-weight: 600;
}

.notebook-item .icon {
  font-size: 1rem;
  flex-shrink: 0;
}

.notebook-item .name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.notebook-item .count {
  font-size: 0.75rem;
  background: var(--bg-input);
  color: var(--text-tertiary);
  padding: 2px 8px;
  border-radius: 10px;
  min-width: 22px;
  text-align: center;
}

.notebook-item .delete-btn {
  opacity: 0;
  font-size: 0.8rem;
  color: var(--danger);
  cursor: pointer;
  padding: 2px 4px;
  border-radius: 4px;
  transition: all var(--transition);
  background: none;
  border: none;
  line-height: 1;
}

.notebook-item:hover .delete-btn {
  opacity: 0.7;
}

.notebook-item .delete-btn:hover {
  opacity: 1;
  background: var(--danger-light);
}

.add-notebook-btn {
  margin: 6px 10px;
  padding: 9px 12px;
  border-radius: var(--radius);
  border: 1px dashed var(--border-color);
  background: transparent;
  cursor: pointer;
  color: var(--text-tertiary);
  font-size: 0.85rem;
  transition: all var(--transition);
  text-align: center;
}

.add-notebook-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-light);
}
</style>
