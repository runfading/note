/** localStorage 封装 —— 仅用于主题等前端配置持久化 */

const THEME_KEY = 'siyi_theme'

export function getTheme(): string | null {
  return localStorage.getItem(THEME_KEY)
}

export function setTheme(theme: 'light' | 'dark'): void {
  localStorage.setItem(THEME_KEY, theme)
}
