/**
 * 笔记-标签关联 API
 * Tauri v2 参数传递规则：Rust 参数名作为顶层 key，嵌套 struct 字段保持 snake_case
 */
import { request } from '../index'
import type { TagsUsedTimesQuery, TagsUsedTimesResponse } from '@/types/note-tag'

/** 给笔记添加标签 */
export function addNoteTag(noteId: number, tagName: string): Promise<null> {
  return request<null>('add_note_tag', {
    noteId,
    tag: { name: tagName },
  })
}

/** 删除笔记的标签 */
export function deleteNoteTag(noteId: number, tagName: string): Promise<null> {
  return request<null>('delete_note_tag', {
    noteId,
    tag: tagName,
  })
}

/** 查询标签使用次数 */
export function queryTagUsedTimes(query: TagsUsedTimesQuery): Promise<TagsUsedTimesResponse[]> {
  return request<TagsUsedTimesResponse[]>('query_tag_used_times', {
    query: {
      ids: query.ids,
    },
  })
}
