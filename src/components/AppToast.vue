<script setup lang="ts">
/**
 * Toast 提示组件
 */
import { useToast } from '@/composables/useDialog'

const { toastMsg, toastVisible } = useToast()
</script>

<template>
  <div class="toast" :class="{ show: toastVisible }">{{ toastMsg }}</div>
</template>

<style scoped>
.toast {
  position: fixed;
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%) translateY(100px);
  background: #333;
  color: #fff;
  padding: 10px 22px;
  border-radius: 20px;
  font-size: 0.85rem;
  z-index: 999;
  pointer-events: none;
  transition: transform 0.35s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  box-shadow: var(--shadow-lg);
}
.toast.show {
  transform: translateX(-50%) translateY(0);
}
</style>
