/**
 * 自动保存 composable
 * - 输入 800ms 后自动触发保存
 * - 每 30s 强制保存一次
 * - 页面关闭前保存
 */
import { ref, onMounted, onUnmounted } from 'vue'

export function useAutoSave(saveFn: () => void | Promise<void>, delay = 800) {
  let saveTimeout: ReturnType<typeof setTimeout> | null = null
  let forceInterval: ReturnType<typeof setInterval> | null = null

  const lastSaved = ref<string>('—')

  /** 触发防抖保存 */
  function triggerSave(): void {
    if (saveTimeout) clearTimeout(saveTimeout)
    saveTimeout = setTimeout(async () => {
      await saveFn()
      lastSaved.value = formatNow()
    }, delay)
  }

  /** 立即保存（跳过防抖） */
  async function saveNow(): Promise<void> {
    cancelPending()
    await saveFn()
    lastSaved.value = formatNow()
  }

  /** 取消待处理的防抖保存 */
  function cancelPending(): void {
    if (saveTimeout) {
      clearTimeout(saveTimeout)
      saveTimeout = null
    }
  }

  function formatNow(): string {
    const d = new Date()
    const month = d.getMonth() + 1
    const day = d.getDate()
    const hour = d.getHours().toString().padStart(2, '0')
    const min = d.getMinutes().toString().padStart(2, '0')
    return `已保存 ${month}/${day} ${hour}:${min}`
  }

  onMounted(() => {
    // 每 30 秒强制保存
    forceInterval = setInterval(() => {
      saveFn()
      lastSaved.value = formatNow()
    }, 30000)

    // 页面关闭前保存
    window.addEventListener('beforeunload', onBeforeUnload)
  })

  onUnmounted(() => {
    if (forceInterval) clearInterval(forceInterval)
    window.removeEventListener('beforeunload', onBeforeUnload)
  })

  function onBeforeUnload(): void {
    saveFn()
  }

  return { lastSaved, triggerSave, saveNow, cancelPending }
}
