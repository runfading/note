<script setup lang="ts">
/**
 * 搜索框组件
 */
import { ref, watch } from 'vue'

const props = defineProps<{ modelValue?: string; placeholder?: string }>()
const emit = defineEmits<{ 'update:modelValue': [value: string] }>()

const query = ref(props.modelValue ?? '')

watch(() => props.modelValue, (v) => {
  query.value = v ?? ''
})

function onInput(e: Event): void {
  const target = e.target as HTMLInputElement
  query.value = target.value
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div class="search-wrap">
    <span class="search-icon">🔍</span>
    <input
      type="text"
      class="search-input"
      :value="query"
      :placeholder="placeholder ?? '搜索笔记...'"
      @input="onInput"
    />
  </div>
</template>

<style scoped>
.search-wrap {
  position: relative;
}
.search-input {
  width: 100%;
  padding: 9px 12px 9px 34px;
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: 0.85rem;
  outline: none;
  transition: all var(--transition);
}
.search-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-light);
}
.search-input::placeholder {
  color: var(--text-placeholder);
}
.search-icon {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-tertiary);
  font-size: 0.9rem;
  pointer-events: none;
}
</style>
