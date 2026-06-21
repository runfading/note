/** 后端统一响应包装 */
export interface ApiResponse<T> {
  code: number
  message: string
  data: T | null
}

/** 分页数据 */
export interface ApiPageData<T> {
  list: T[]
  page_num: number
  page_size: number
  total: number
}

/** 通用分页查询参数 */
export interface ApiPageQuery {
  page_num: number
  page_size: number
}
