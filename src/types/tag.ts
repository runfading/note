/** 标签信息（对应 Rust TagInfo） */
export interface TagInfo {
  id: number
  name: string
}

/** 创建标签（对应 Rust CreateTag） */
export interface CreateTag {
  name: string
}

/** 搜索标签（对应 Rust SearchTagsQuery） */
export interface SearchTagsQuery {
  keyword: string
}

/** 按标签分页查询笔记（对应 Rust PageNotesByTagQuery） */
export interface PageNotesByTagQuery {
  page_num?: number
  page_size?: number
}
