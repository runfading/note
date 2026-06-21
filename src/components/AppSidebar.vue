<script setup lang="ts">
/**
 * 左侧边栏 —— Logo + 主题切换 + 笔记本列表 + 标签汇总
 */
import { useAppStore } from '@/stores/app'
import NotebookList from './NotebookList.vue'
import TagChips from './TagChips.vue'

const appStore = useAppStore()
</script>

<template>
  <aside class="sidebar">
    <!-- 顶部：Logo + 主题切换 -->
    <div class="sidebar-header">
      <div class="logo">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="3" />
          <line x1="8" y1="8" x2="16" y2="8" />
          <line x1="8" y1="12" x2="16" y2="12" />
          <line x1="8" y1="16" x2="12" y2="16" />
        </svg>
        <span>思绪</span>
      </div>
      <button class="theme-toggle" :title="appStore.isDark ? '切换亮色' : '切换暗色'" @click="appStore.toggleTheme()">
        {{ appStore.isDark ? '☀️' : '🌙' }}
      </button>
    </div>

    <!-- 笔记本分类列表 -->
    <NotebookList />

    <!-- 标签汇总区 -->
    <TagChips />
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  min-width: var(--sidebar-width);
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  height: 100vh;
  transition: all var(--transition);
  z-index: 10;
}
.sidebar-header {
  padding: 20px 20px 16px;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  flex-shrink: 0;
}
.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--accent);
  letter-spacing: -0.3px;
  white-space: nowrap;
}
.logo svg {
  width: 28px;
  height: 28px;
  flex-shrink: 0;
}
.theme-toggle {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  cursor: pointer;
  font-size: 1.1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition);
  flex-shrink: 0;
  color: var(--text-secondary);
}
.theme-toggle:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--accent);
}
</style>
