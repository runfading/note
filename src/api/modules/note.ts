/**
 * 笔记 API
 * Tauri v2 参数传递规则：Rust 参数名作为顶层 key，嵌套 struct 字段保持 snake_case
 */
import { request } from '../index'
import type { ApiPageData } from '@/types/api'
import type { NoteInfo, CreateNote, UpdateNote, PageNotesQuery, SearchNotesQuery } from '@/types/note'

/** 分页查询笔记 */
export function pageNotes(query: PageNotesQuery): Promise<ApiPageData<NoteInfo>> {
  return request<ApiPageData<NoteInfo>>('page_notes', {
    query: {
      page_num: query.page_num ?? 1,
      page_size: query.page_size ?? 10,
      notebook_id: query.notebook_id ?? null,
    },
  })
}

/** 搜索笔记 */
export function searchNotes(query: SearchNotesQuery): Promise<ApiPageData<NoteInfo>> {
  return request<ApiPageData<NoteInfo>>('search_notes', {
    query: {
      keyword: query.keyword,
      page_num: query.page_num ?? 1,
      page_size: query.page_size ?? 10,
    },
  })
}

/** 创建笔记 */
export function createNote(data: CreateNote): Promise<number> {
  return request<number>('create_note', {
    create: {
      notebook_id: data.notebook_id,
      title: data.title,
      content: data.content,
    },
  })
}

/** 删除笔记 */
export function removeNote(id: number): Promise<null> {
  return request<null>('remove_note', { id })
}

/** 更新笔记 */
export function updateNote(data: UpdateNote): Promise<null> {
  return request<null>('update_note', {
    update: {
      note_id: data.note_id,
      notebook_id: data.notebook_id ?? null,
      title: data.title ?? null,
      content: data.content ?? null,
    },
  })
}

/** 笔记详情 */
export function noteDetail(id: number): Promise<NoteInfo | null> {
  return request<NoteInfo | null>('note_detail', { id })
}
