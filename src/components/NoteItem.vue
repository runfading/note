<script setup lang="ts">
/**
 * 单条笔记摘要卡片
 */
import type { NoteInfo } from '@/types/note'
import { formatTime, stripHtml, escapeHtml } from '@/utils/format'

const props = defineProps<{
  note: NoteInfo
  active?: boolean
}>()

const emit = defineEmits<{
  select: [noteId: number]
}>()

const preview = stripHtml(props.note.content ?? '').slice(0, 60) || '(空内容)'
const timeStr = formatTime(props.note.updated_at ?? Date.now())
</script>

<template>
  <div class="note-item" :class="{ active }" @click="emit('select', note.id)">
    <div class="note-title">{{ escapeHtml(note.title) || '无标题' }}</div>
    <div class="note-preview">{{ escapeHtml(preview) }}</div>
    <div class="note-meta">
      <span>{{ timeStr }}</span>
      <div class="note-tags-mini">
        <span v-for="tag in note.tags" :key="tag.id" class="mini-tag">{{ escapeHtml(tag.name) }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.note-item {
  padding: 12px 14px;
  border-radius: var(--radius);
  cursor: pointer;
  transition: all var(--transition);
  border: 1px solid transparent;
  margin-bottom: 3px;
  position: relative;
}
.note-item:hover {
  background: var(--bg-hover);
  border-color: var(--border-light);
}
.note-item.active {
  background: var(--bg-active);
  border-color: var(--accent);
  box-shadow: var(--shadow-sm);
}
.note-title {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.note-preview {
  font-size: 0.78rem;
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 6px;
}
.note-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.7rem;
  color: var(--text-tertiary);
}
.note-tags-mini {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
.mini-tag {
  font-size: 0.65rem;
  padding: 2px 7px;
  border-radius: 8px;
  background: var(--tag-bg);
  color: var(--tag-text);
  white-space: nowrap;
}
</style>
