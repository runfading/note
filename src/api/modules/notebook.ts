/**
 * 笔记本 API
 * Tauri v2 参数传递规则：Rust 参数名作为顶层 key，嵌套 struct 字段保持 snake_case
 */
import { request } from '../index'
import type { ApiPageData } from '@/types/api'
import type { NotebookInfo, CreateNotebook } from '@/types/notebook'
import type { ApiPageQuery } from '@/types/api'

/** 分页查询笔记本 */
export function pageNotebooks(query: ApiPageQuery): Promise<ApiPageData<NotebookInfo>> {
  return request<ApiPageData<NotebookInfo>>('plugin:notebooks|page_notebooks', {
    query: {
      page_num: query.page_num,
      page_size: query.page_size,
    },
  })
}

/** 创建笔记本 */
export function createNotebook(data: CreateNotebook): Promise<number> {
  return request<number>('plugin:notebooks|create_notebook', {
    create: {
      name: data.name,
    },
  })
}

/** 删除笔记本 */
export function removeNotebook(id: number): Promise<null> {
  return request<null>('plugin:notebooks|remove_notebook', { id })
}
