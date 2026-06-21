<script setup lang="ts">
/**
 * 主页 —— 使用默认三栏布局
 */
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import AppToast from '@/components/AppToast.vue'
import AppDialog from '@/components/AppDialog.vue'
</script>

<template>
  <DefaultLayout />
  <AppToast />
  <AppDialog />
</template>
