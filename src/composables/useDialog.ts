/**
 * 对话框 & Toast 管理 composable
 * 提供响应式的 Toast 和 Dialog 控制
 */
import { ref } from 'vue'

// ========== Toast ==========

const toastMsg = ref('')
const toastVisible = ref(false)
let toastTimer: ReturnType<typeof setTimeout> | null = null

export function useToast() {
  function showToast(msg: string, duration = 2000): void {
    toastMsg.value = msg
    toastVisible.value = true
    if (toastTimer) clearTimeout(toastTimer)
    toastTimer = setTimeout(() => {
      toastVisible.value = false
    }, duration)
  }

  return { toastMsg, toastVisible, showToast }
}

// ========== Dialog ==========

export interface DialogConfig {
  title: string
  defaultValue?: string
  confirmText?: string
  inputLabel?: string
}

const dialogVisible = ref(false)
const dialogConfig = ref<DialogConfig>({ title: '' })
let dialogResolve: ((value: string | null) => void) | null = null

export function useDialog() {
  /** 打开输入对话框，返回用户输入的值或 null（取消） */
  function showDialog(config: DialogConfig): Promise<string | null> {
    return new Promise((resolve) => {
      dialogConfig.value = {
        ...config,
        confirmText: config.confirmText ?? '确定',
        inputLabel: config.inputLabel ?? '输入名称...',
      }
      dialogVisible.value = true
      dialogResolve = resolve
    })
  }

  /** 确认对话框返回 */
  function confirmDialog(value: string): void {
    dialogVisible.value = false
    if (dialogResolve) {
      dialogResolve(value)
      dialogResolve = null
    }
  }

  /** 取消对话框 */
  function cancelDialog(): void {
    dialogVisible.value = false
    if (dialogResolve) {
      dialogResolve(null)
      dialogResolve = null
    }
  }

  return { dialogVisible, dialogConfig, showDialog, confirmDialog, cancelDialog }
}

// ========== Confirm Dialog ==========

interface ConfirmConfig {
  title: string
  message: string
}

const confirmVisible = ref(false)
const confirmConfig = ref<ConfirmConfig>({ title: '', message: '' })
let confirmResolve: ((value: boolean) => void) | null = null

export function useConfirmDialog() {
  function showConfirm(config: ConfirmConfig): Promise<boolean> {
    return new Promise((resolve) => {
      confirmConfig.value = config
      confirmVisible.value = true
      confirmResolve = resolve
    })
  }

  function confirmOk(): void {
    confirmVisible.value = false
    if (confirmResolve) {
      confirmResolve(true)
      confirmResolve = null
    }
  }

  function confirmCancel(): void {
    confirmVisible.value = false
    if (confirmResolve) {
      confirmResolve(false)
      confirmResolve = null
    }
  }

  return { confirmVisible, confirmConfig, showConfirm, confirmOk, confirmCancel }
}
