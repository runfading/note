/**
 * HTML 工具函数 —— 从 HTML 原型迁移
 */

/** 生成唯一 ID */
export function generateId(): string {
  return 'id_' + Date.now() + '_' + Math.random().toString(36).slice(2, 8)
}

/** 格式化时间戳为相对时间 */
export function formatTime(ts: number): string {
  const d = new Date(ts)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return Math.floor(diff / 60000) + ' 分钟前'
  if (diff < 86400000) return Math.floor(diff / 3600000) + ' 小时前'
  const month = d.getMonth() + 1
  const day = d.getDate()
  const hour = d.getHours().toString().padStart(2, '0')
  const min = d.getMinutes().toString().padStart(2, '0')
  return `${month}/${day} ${hour}:${min}`
}

/** 移除 HTML 标签，提取纯文本 */
export function stripHtml(html: string): string {
  const tmp = document.createElement('div')
  tmp.innerHTML = html
  return tmp.textContent || tmp.innerText || ''
}

/** 统计字数（去除空白） */
export function countWords(text: string): number {
  return text.replace(/\s+/g, '').length
}

/** HTML 转义 */
export function escapeHtml(str: string): string {
  const div = document.createElement('div')
  div.textContent = str
  return div.innerHTML
}
