<![CDATA[
<script setup lang="ts">
import { useToast } from '../composables/useToast';

const { showSuccessToast, showInfoToast, showErrorToast } = useToast();

function testAll() {
  showInfoToast('信息toast');
  setTimeout(() => showSuccessToast('成功toast'), 500);
  setTimeout(() => showErrorToast('错误toast'), 1000);
}
</script>

<template>
  <div class="p-4 bg-white rounded shadow">
    <button @click="testAll" class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
      测试所有Toast
    </button>
    <p class="text-sm text-gray-500 mt-2">点击后左下角应显示3个toast</p>
  </div>
</template>

<style scoped>
/* empty */
</style>
]]>
