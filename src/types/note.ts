import type { TagInfo } from './tag'

/** 笔记信息（对应 Rust NoteInfo） */
export interface NoteInfo {
  id: number
  notebook_id: number
  title: string
  content?: string
  tags: TagInfo[]
  created_at?: number
  updated_at?: number
}

/** 创建笔记（对应 Rust CreateNote） */
export interface CreateNote {
  notebook_id: number
  title: string
  content: string
}

/** 更新笔记（对应 Rust UpdateNote） */
export interface UpdateNote {
  note_id: number
  notebook_id?: number
  title?: string
  content?: string
}

/** 分页查询笔记（对应 Rust PageNotesQuery） */
export interface PageNotesQuery {
  page_num?: number
  page_size?: number
  notebook_id?: number
}

/** 搜索笔记（对应 Rust SearchNotesQuery） */
export interface SearchNotesQuery {
  page_num?: number
  page_size?: number
  keyword: string
}
