<script setup lang="ts">
/**
 * Milkdown Crepe 编辑器 —— 直接操作 Crepe 实例
 * 通过 :key 重建来切换笔记
 */
import { ref, onMounted, onUnmounted } from 'vue'
import { Crepe } from '@milkdown/crepe'
import '@milkdown/crepe/theme/common/style.css'
import '@milkdown/crepe/theme/frame.css'

const props = defineProps<{ modelValue: string }>()

const containerRef = ref<HTMLDivElement | null>(null)
const ready = ref(false)
let crepe: Crepe | null = null

onMounted(async () => {
  if (!containerRef.value) return
  crepe = new Crepe({
    root: containerRef.value,
    defaultValue: props.modelValue, 
  })
  await crepe.create()
  ready.value = true
})

onUnmounted(async () => {
  if (crepe) {
    await crepe.destroy()
    crepe = null
    ready.value = false
  }
})

/** 获取 markdown 内容 */
function getMarkdown(): string {
  return crepe?.getMarkdown() ?? ''
}

defineExpose({ getMarkdown, ready })
</script>

<template>
  <div ref="containerRef" class="milkdown-root"></div>
</template>

<style scoped>
.milkdown-root {
  height: 100%;
  overflow: hidden;
}
</style>
