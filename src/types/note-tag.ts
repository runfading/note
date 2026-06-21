/** 添加笔记标签（对应 Rust AddNoteTag） */
export interface AddNoteTag {
  name: string
}

/** 查询标签使用次数（对应 Rust TagsUsedTimesQuery） */
export interface TagsUsedTimesQuery {
  ids: number[]
}

/** 标签使用次数响应（对应 Rust TagsUsedTimesResponse） */
export interface TagsUsedTimesResponse {
  id: number
  use_count: number
}
