<script setup lang="ts">
/**
 * 编辑器内标签展示 —— 显示当前笔记的标签，支持移除
 */
import { useNoteStore } from '@/stores/note'
import { useTagStore } from '@/stores/tag'
import { useToast } from '@/composables/useDialog'
import { escapeHtml } from '@/utils/format'

const noteStore = useNoteStore()
const tagStore = useTagStore()
const { showToast } = useToast()

async function removeTag(tagName: string): Promise<void> {
  if (!noteStore.activeNoteId) return
  const ok = await noteStore.removeTag(noteStore.activeNoteId, tagName)
  if (ok) {
    // 刷新活跃笔记
    const detail = await noteStore.fetchNoteDetail(noteStore.activeNoteId)
    if (detail && noteStore.activeNote) {
      noteStore.activeNote.tags = detail.tags
    }
    await tagStore.fetchRecentTags()
    showToast('标签已移除')
  }
}
</script>

<template>
  <div class="editor-tags-display" v-if="noteStore.activeNote">
    <span
      v-for="tag in noteStore.activeNote.tags"
      :key="tag.id"
      class="editor-tag"
    >
      {{ escapeHtml(tag.name) }}
      <span class="remove-tag" @click="removeTag(tag.name)">×</span>
    </span>
  </div>
</template>

<style scoped>
.editor-tags-display {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  align-items: center;
}
.editor-tag {
  font-size: 0.7rem;
  padding: 3px 9px;
  border-radius: 12px;
  background: var(--tag-bg);
  color: var(--tag-text);
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: default;
  white-space: nowrap;
}
.remove-tag {
  cursor: pointer;
  font-size: 0.8rem;
  opacity: 0.6;
  transition: opacity var(--transition);
  line-height: 1;
}
.remove-tag:hover {
  opacity: 1;
  color: var(--danger);
}
</style>
