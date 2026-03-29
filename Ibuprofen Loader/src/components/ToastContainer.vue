<script setup lang="ts">
import { toasts, removeToast } from '../composables/useToast';
</script>

<template>
  <div class="toast-container">
    <transition-group name="list" tag="div">
      <div
        v-for="toast in toasts" 
        :key="toast.id"
        class="toast"
        :class="toast.type"
      >
        {{ toast.message }}
        <button @click="removeToast(toast.id)" class="close">×</button>
      </div>
    </transition-group>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 16px;
  left: 16px;
  z-index: 9999;
  width: 360px;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease-out;
}

.list-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(-100%);
}

.toast {
  width: 360px !important;
  background: white;
  border-radius: 6px;
  padding: 12px 16px 12px 16px;
  box-shadow: 0 4px 16px rgba(0,0,0,0.12);
  border-left: 4px solid;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 14px;
  line-height: 1.3;
}

.close {
  width: 24px;
  height: 24px;
  border: none;
  background: none;
  border-radius: 50%;
  cursor: pointer;
  font-size: 16px;
  color: #9ca3af;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-left: 8px;
  opacity: 0.8;
  transition: all 0.2s;
}

.close:hover {
  background: rgba(0,0,0,0.08);
  opacity: 1;
}

.toast.success { border-left-color: #10b981; }
.toast.error { border-left-color: #ef4444; }
.toast.info { border-left-color: #3b82f6; }
.toast.warning { border-left-color: #f59e0b; }
</style>
