<script setup lang="ts">
/**
 * 根组件
 */
import { initTheme } from '@/composables/useTheme'

initTheme()
</script>

<template>
  <router-view />
</template>
