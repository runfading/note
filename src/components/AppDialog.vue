<script setup lang="ts">
/**
 * 通用对话框组件（输入框 + 确认/取消）
 */
import { ref, nextTick, watch } from 'vue'
import { useDialog, useConfirmDialog } from '@/composables/useDialog'

const { dialogVisible, dialogConfig, confirmDialog, cancelDialog } = useDialog()
const { confirmVisible, confirmConfig, confirmOk, confirmCancel } = useConfirmDialog()

const inputValue = ref('')
const inputEl = ref<HTMLInputElement | null>(null)

// 对话框打开时聚焦输入框
watch(dialogVisible, async (v) => {
  if (v) {
    inputValue.value = dialogConfig.value.defaultValue ?? ''
    await nextTick()
    inputEl.value?.focus()
    inputEl.value?.select()
  }
})

function onConfirm(): void {
  const val = inputValue.value.trim()
  if (val) {
    confirmDialog(val)
  } else if (inputEl.value) {
    inputEl.value.style.borderColor = 'var(--danger)'
    setTimeout(() => {
      if (inputEl.value) inputEl.value.style.borderColor = ''
    }, 800)
    inputEl.value.focus()
  }
}

function onKeydown(e: KeyboardEvent): void {
  if (e.key === 'Enter') onConfirm()
  if (e.key === 'Escape') cancelDialog()
}
</script>

<template>
  <!-- 输入对话框 -->
  <div v-if="dialogVisible" class="dialog-overlay" @click.self="cancelDialog">
    <div class="dialog-box">
      <h3>{{ dialogConfig.title }}</h3>
      <input
        ref="inputEl"
        v-model="inputValue"
        type="text"
        :placeholder="dialogConfig.inputLabel ?? '输入名称...'"
        @keydown="onKeydown"
      />
      <div class="dialog-buttons">
        <button class="btn-cancel" @click="cancelDialog">取消</button>
        <button class="btn-primary" @click="onConfirm">{{ dialogConfig.confirmText ?? '确定' }}</button>
      </div>
    </div>
  </div>

  <!-- 确认对话框 -->
  <div v-if="confirmVisible" class="dialog-overlay" @click.self="confirmCancel">
    <div class="dialog-box">
      <h3>{{ confirmConfig.title }}</h3>
      <p class="confirm-msg">{{ confirmConfig.message }}</p>
      <div class="dialog-buttons">
        <button class="btn-cancel" @click="confirmCancel">取消</button>
        <button class="btn-danger" @click="confirmOk">确认删除</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s ease;
}
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
.dialog-box {
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  padding: 24px;
  width: 380px;
  max-width: 90vw;
  box-shadow: var(--shadow-lg);
  border: 1px solid var(--border-color);
}
.dialog-box h3 {
  margin-bottom: 14px;
  font-size: 1rem;
  color: var(--text-primary);
}
.dialog-box input {
  width: 100%;
  padding: 10px 14px;
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: 0.9rem;
  outline: none;
  margin-bottom: 16px;
}
.dialog-box input:focus {
  border-color: var(--accent);
}
.confirm-msg {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin-bottom: 16px;
}
.dialog-buttons {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}
.dialog-buttons button {
  padding: 8px 18px;
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
  cursor: pointer;
  font-size: 0.85rem;
  transition: all var(--transition);
  background: var(--bg-secondary);
  color: var(--text-primary);
}
.btn-primary {
  background: var(--accent) !important;
  color: #fff !important;
  border-color: var(--accent) !important;
}
.btn-primary:hover {
  background: var(--accent-hover) !important;
}
.btn-danger {
  background: var(--danger) !important;
  color: #fff !important;
  border-color: var(--danger) !important;
}
.btn-danger:hover {
  background: var(--danger-hover) !important;
}
</style>
