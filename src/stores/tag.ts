/**
 * 标签状态管理
 */
import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as tagApi from '@/api/modules/tag'
import * as noteTagApi from '@/api/modules/note-tag'
import type { TagInfo } from '@/types/tag'

export const useTagStore = defineStore('tag', () => {
  const tags = ref<TagInfo[]>([])
  const activeTagFilter = ref<string | null>(null)
  /** tag id → use_count */
  const tagUsageMap = ref<Record<number, number>>({})

  /** 加载最近标签 */
  async function fetchRecentTags(): Promise<void> {
    try {
      tags.value = await tagApi.recentTags()
    } catch (e) {
      console.error('加载标签失败:', e)
    }
  }

  /** 搜索标签 */
  async function searchTags(keyword: string): Promise<TagInfo[]> {
    try {
      return await tagApi.searchTags({ keyword })
    } catch (e) {
      console.error('搜索标签失败:', e)
      return []
    }
  }

  /** 创建标签 */
  async function createTag(name: string): Promise<number | null> {
    try {
      const id = await tagApi.createTag({ name })
      await fetchRecentTags()
      return id
    } catch (e) {
      console.error('创建标签失败:', e)
      return null
    }
  }

  /** 删除标签 */
  async function deleteTag(name: string): Promise<boolean> {
    try {
      await tagApi.deleteTag(name)
      await fetchRecentTags()
      return true
    } catch (e) {
      console.error('删除标签失败:', e)
      return false
    }
  }

  /** 获取标签使用次数 */
  async function fetchTagUsage(ids: number[]): Promise<void> {
    if (ids.length === 0) return
    try {
      const result = await noteTagApi.queryTagUsedTimes({ ids })
      for (const item of result) {
        tagUsageMap.value[item.id] = item.use_count
      }
    } catch (e) {
      console.error('查询标签使用次数失败:', e)
    }
  }

  /** 设置标签筛选 */
  function setTagFilter(tag: string | null): void {
    if (activeTagFilter.value === tag) {
      activeTagFilter.value = null
    } else {
      activeTagFilter.value = tag
    }
  }

  /** 根据名称查找标签 ID */
  function getTagIdByName(name: string): number | null {
    return tags.value.find(t => t.name === name)?.id ?? null
  }

  return {
    tags,
    activeTagFilter,
    tagUsageMap,
    fetchRecentTags,
    searchTags,
    createTag,
    deleteTag,
    fetchTagUsage,
    setTagFilter,
    getTagIdByName,
  }
})
