/** 笔记本信息（对应 Rust NotebookInfo） */
export interface NotebookInfo {
  id: number
  name: string
}

/** 创建笔记本（对应 Rust CreateNotebook） */
export interface CreateNotebook {
  name: string
}
