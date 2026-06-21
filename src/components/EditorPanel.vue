<script setup lang="ts">
/**
 * 编辑器面板 —— 标题 + Milkdown Markdown 编辑器 + 标签栏 + 状态栏
 */
import { ref, watch, computed, nextTick } from 'vue'
import { useNoteStore } from '@/stores/note'
import { useTagStore } from '@/stores/tag'
import { useAppStore } from '@/stores/app'
import { useToast } from '@/composables/useDialog'
import { useAutoSave } from '@/composables/useAutoSave'
import { countWords } from '@/utils/format'
import EditorTags from './EditorTags.vue'
import MilkdownEditor from './MilkdownEditor.vue'

const noteStore = useNoteStore()
const tagStore = useTagStore()
const appStore = useAppStore()
const { showToast } = useToast()

const titleInput = ref('')
const editorRef = ref<InstanceType<typeof MilkdownEditor> | null>(null)
const isEditing = ref(false)

/** 用于 MilkdownEditor 的 key —— 切换笔记时重建编辑器 */
const editorKey = ref(0)

/** 传给 MilkdownEditor 的初始 markdown 内容 */
const editorContent = ref('')

/** 是否显示编辑器内容 */
const showEditor = computed(() => noteStore.activeNote !== null)

/** 字数统计 */
const wordCount = computed(() => {
  if (!noteStore.activeNote?.content) return '0 字'
  return countWords(noteStore.activeNote.content) + ' 字'
})

/** 同步编辑内容到 store 并保存到后端 */
async function syncAndSave(): Promise<void> {
  if (!noteStore.activeNoteId) return
  const title = titleInput.value.trim()
  const content = editorRef.value?.getMarkdown() ?? ''
  noteStore.updateLocalNote(noteStore.activeNoteId, { title, content })
  await noteStore.updateNote(noteStore.activeNoteId, { title, content })
}

const { lastSaved, triggerSave, cancelPending } = useAutoSave(syncAndSave)

/** 记录当前加载的笔记 ID，防止切换时覆盖 */
let loadingNoteId: number | null = null

/** 监听当前笔记变化：先保存旧笔记，再加载新笔记 */
watch(
  () => noteStore.activeNoteId,
  async (newId, oldId) => {
    // 取消旧的防抖
    cancelPending()

    // 保存旧笔记
    if (oldId && oldId !== loadingNoteId) {
      const oldTitle = titleInput.value.trim()
      const oldContent = editorRef.value?.getMarkdown() ?? ''
      if (oldTitle || oldContent) {
        noteStore.updateLocalNote(oldId, { title: oldTitle, content: oldContent })
        await noteStore.updateNote(oldId, { title: oldTitle, content: oldContent })
      }
    }

    if (newId) {
      loadingNoteId = newId
      const detail = await noteStore.fetchNoteDetail(newId)
      if (detail && loadingNoteId === newId) {
        titleInput.value = detail.title
        editorContent.value = detail.content ?? ''
        editorKey.value++
        await nextTick()
      }
      isEditing.value = true
      lastSaved.value = '—'
      loadingNoteId = null
    } else {
      isEditing.value = false
      titleInput.value = ''
      editorContent.value = ''
    }
  },
  { immediate: true },
)

/** 标题输入 */
function onTitleInput(): void {
  triggerSave()
}

/** 编辑器内容变更（未来可用于实时同步） */
function onEditorChange(): void {
  triggerSave()
}

/** 添加标签 */
const tagInput = ref('')

async function onAddTag(): Promise<void> {
  const val = tagInput.value.trim()
  if (!val || !noteStore.activeNoteId) return
  const ok = await noteStore.addTag(noteStore.activeNoteId, val)
  if (ok) {
    await tagStore.fetchRecentTags()
    const detail = await noteStore.fetchNoteDetail(noteStore.activeNoteId)
    if (detail && noteStore.activeNote) {
      noteStore.activeNote.tags = detail.tags
    }
    tagInput.value = ''
    showToast(`标签"${val}"已添加`)
  }
}

function onTagKeydown(e: KeyboardEvent): void {
  if (e.key === 'Enter') onAddTag()
}
</script>

<template>
  <div class="editor-panel" :class="{ 'show-mobile': appStore.mobileEditorVisible }">
    <!-- 空状态 -->
    <div v-if="!showEditor" class="editor-empty">
      <span class="big-icon">📝</span>
      <span>选择一篇笔记开始编辑</span>
      <span class="sub-hint">或点击 + 创建新笔记</span>
    </div>

    <!-- 编辑状态 -->
    <div v-else class="editor-content">
      <!-- 标题 + 标签栏 -->
      <div class="editor-top">
        <input
          type="text"
          class="title-input"
          v-model="titleInput"
          placeholder="笔记标题..."
          @input="onTitleInput"
        />

        <div class="tag-bar">
          <EditorTags />
          <div class="tag-input-area">
            <input
              type="text"
              class="tag-input"
              v-model="tagInput"
              placeholder="添加标签..."
              @keydown="onTagKeydown"
            />
            <button class="tag-add-btn" @click="onAddTag">+标签</button>
          </div>
        </div>
      </div>

      <!-- Milkdown Markdown 编辑器 -->
      <div class="editor-body">
        <MilkdownEditor
          ref="editorRef"
          :key="editorKey"
          :model-value="editorContent"
          @update:model-value="onEditorChange"
        />
      </div>

      <!-- 状态栏 -->
      <div class="editor-statusbar">
        <span class="save-indicator"><span class="save-dot"></span> 自动保存已启用</span>
        <span>{{ wordCount }}</span>
        <span class="last-saved">{{ lastSaved }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-editor);
  transition: background var(--transition);
  min-width: 0;
}
.editor-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
  font-size: 0.9rem;
  flex-direction: column;
  gap: 8px;
  user-select: none;
}
.big-icon {
  font-size: 5rem;
  opacity: 0.3;
  margin-bottom: 8px;
}
.sub-hint {
  font-size: 0.75rem;
}
.editor-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.editor-top {
  flex-shrink: 0;
  border-bottom: 1px solid var(--border-light);
}
.title-input {
  width: 100%;
  border: none;
  font-size: 1.6rem;
  font-weight: 700;
  padding: 16px 24px 8px;
  background: transparent;
  color: var(--text-primary);
  outline: none;
  letter-spacing: -0.3px;
}
.title-input::placeholder {
  color: var(--text-placeholder);
  font-weight: 400;
}

/* 标签栏 */
.tag-bar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 24px 10px;
  flex-wrap: wrap;
}
.tag-input-area {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: auto;
  flex-shrink: 0;
}
.tag-input {
  width: 100px;
  padding: 4px 10px;
  border-radius: 14px;
  border: 1px solid var(--border-color);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: 0.75rem;
  outline: none;
  transition: all var(--transition);
}
.tag-input:focus {
  border-color: var(--accent);
  width: 140px;
}
.tag-add-btn {
  font-size: 0.7rem;
  padding: 4px 10px;
  border-radius: 14px;
  border: 1px solid var(--accent);
  background: var(--accent-light);
  color: var(--accent);
  cursor: pointer;
  white-space: nowrap;
  transition: all var(--transition);
}
.tag-add-btn:hover {
  background: var(--accent);
  color: #fff;
}

/* 编辑器主体 —— Milkdown 编辑器占据剩余空间 */
.editor-body {
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.editor-statusbar {
  height: 28px;
  border-top: 1px solid var(--border-light);
  display: flex;
  align-items: center;
  padding: 0 16px;
  font-size: 0.7rem;
  color: var(--text-tertiary);
  gap: 12px;
  flex-shrink: 0;
  background: var(--bg-editor);
}
.save-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
}
.save-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #4caf50;
  animation: pulse-dot 2s infinite;
}
@keyframes pulse-dot {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}
.last-saved {
  margin-left: auto;
}
</style>
