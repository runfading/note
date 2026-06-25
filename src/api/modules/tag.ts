/**
 * 标签 API
 * Tauri v2 参数传递规则：Rust 参数名作为顶层 key，嵌套 struct 字段保持 snake_case
 */
import { request } from '../index'
import type { ApiPageData } from '@/types/api'
import type { NoteInfo } from '@/types/note'
import type { TagInfo, CreateTag, SearchTagsQuery, PageNotesByTagQuery } from '@/types/tag'

/** 最近标签 */
export function recentTags(): Promise<TagInfo[]> {
  return request<TagInfo[]>('plugin:tags|recent_tags')
}

/** 创建标签 */
export function createTag(data: CreateTag): Promise<number> {
  return request<number>('plugin:tags|create_tag', {
    create: {
      name: data.name,
    },
  })
}

/** 搜索标签 */
export function searchTags(query: SearchTagsQuery): Promise<TagInfo[]> {
  return request<TagInfo[]>('plugin:tags|search_tags', {
    query: {
      keyword: query.keyword,
    },
  })
}

/** 按标签分页查询笔记 */
export function pageNotesByTag(tagId: number, query: PageNotesByTagQuery): Promise<ApiPageData<NoteInfo>> {
  return request<ApiPageData<NoteInfo>>('plugin:tags|page_notes_by_tag', {
    id: tagId,
    query: {
      page_num: query.page_num ?? 1,
      page_size: query.page_size ?? 10,
    },
  })
}

/** 删除标签 */
export function deleteTag(name: string): Promise<null> {
  return request<null>('plugin:tags|delete_tag', { name })
}
