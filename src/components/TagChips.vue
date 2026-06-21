<script setup lang="ts">
/**
 * 标签汇总区 —— 展示所有标签，点击可筛选
 */
import { onMounted } from 'vue'
import { useTagStore } from '@/stores/tag'
import { useNotebookStore } from '@/stores/notebook'
import { useNoteStore } from '@/stores/note'
import * as tagApi from '@/api/modules/tag'

const tagStore = useTagStore()
const notebookStore = useNotebookStore()
const noteStore = useNoteStore()

onMounted(() => {
  tagStore.fetchRecentTags()
})

/** 点击标签筛选 */
async function onTagClick(tagName: string): Promise<void> {
  tagStore.setTagFilter(tagName)
  if (tagStore.activeTagFilter) {
    // 按标签筛选
    notebookStore.setActiveNotebook(null)
    const tagId = tagStore.getTagIdByName(tagName)
    if (tagId) {
      try {
        const result = await tagApi.pageNotesByTag(tagId, { page_num: 1, page_size: 50 })
        noteStore.$patch({ notes: result.list, total: result.total })
      } catch (e) {
        console.error('按标签加载笔记失败:', e)
      }
    }
  } else {
    // 取消筛选，恢复
    tagStore.activeTagFilter = null
    await noteStore.fetchNotesByNotebook(notebookStore.activeNotebookId)
  }
}
</script>

<template>
  <div class="tags-section">
    <span class="section-label">🏷️ 标签</span>
    <div class="tags-list">
      <span v-if="tagStore.tags.length === 0" class="empty-tags">暂无标签</span>
      <span
        v-for="tag in tagStore.tags"
        :key="tag.id"
        class="tag-chip"
        :class="{ active: tagStore.activeTagFilter === tag.name }"
        @click="onTagClick(tag.name)"
      >
        {{ tag.name }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.tags-section {
  border-top: 1px solid var(--border-light);
  padding: 8px 0 12px;
  flex-shrink: 0;
  max-height: 180px;
  display: flex;
  flex-direction: column;
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
.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding: 4px 16px;
  overflow-y: auto;
}
.tag-chip {
  font-size: 0.78rem;
  padding: 4px 10px;
  border-radius: 14px;
  background: var(--tag-bg);
  color: var(--tag-text);
  cursor: pointer;
  transition: all var(--transition);
  white-space: nowrap;
  user-select: none;
  border: 1px solid transparent;
}
.tag-chip:hover {
  border-color: var(--accent);
  background: var(--accent-light);
}
.tag-chip.active {
  background: var(--accent);
  color: #fff;
  border-color: var(--accent);
}
.empty-tags {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  padding: 4px;
}
</style>
