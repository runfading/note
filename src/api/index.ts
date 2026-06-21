/**
 * API 层 —— 封装 Tauri invoke，统一处理 ApiResponse 解包
 */
import { invoke } from '@tauri-apps/api/core'
import type { ApiResponse, ApiPageData } from '@/types/api'

/** 通用 invoke 包装：调用 Tauri command 并解包 ApiResponse.data */
async function request<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const res = await invoke<ApiResponse<T>>(cmd, args)
  if (res.code !== 0) {
    throw new Error(`[${res.code}] ${res.message}`)
  }
  return res.data as T
}

export { request }
export type { ApiPageData }
