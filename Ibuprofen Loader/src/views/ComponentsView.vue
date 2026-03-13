<script setup lang="ts">
import { useAppStore } from "../stores/app";

const store = useAppStore();

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}
</script>

<template>
  <div class="components-view">
    <div class="header">
      <h2>组件管理</h2>
      <button class="refresh-btn" @click="store.scanComponents">
        刷新
      </button>
    </div>

    <div v-if="store.components.length === 0" class="empty-state">
      <p>未发现组件文件</p>
      <p class="hint">请将 .COP 格式的组件文件放入 components 目录</p>
    </div>

    <div v-else class="components-list">
      <div 
        v-for="component in store.components" 
        :key="component.path"
        :class="['component-card', { selected: store.selectedComponent?.path === component.path }]"
        @click="store.selectComponent(component)"
      >
        <div class="component-icon">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-6 10H6v-2h8v2zm4-4H6v-2h12v2z"/>
          </svg>
        </div>
        <div class="component-info">
          <h3>{{ component.name }}</h3>
          <p class="size">{{ formatSize(component.size) }}</p>
        </div>
        <div class="component-status" v-if="store.selectedComponent?.path === component.path">
          <span class="checkmark">✓</span>
        </div>
      </div>
    </div>

    <div v-if="store.selectedComponent && store.componentInfo" class="component-details">
      <h3>组件详情</h3>
      <div class="detail-grid">
        <div class="detail-item">
          <label>名称:</label>
          <span>{{ store.componentInfo.name }}</span>
        </div>
        <div class="detail-item">
          <label>版本:</label>
          <span>{{ store.componentInfo.version }}</span>
        </div>
        <div class="detail-item">
          <label>作者:</label>
          <span>{{ store.componentInfo.author }}</span>
        </div>
        <div class="detail-item">
          <label>描述:</label>
          <span>{{ store.componentInfo.description || '无' }}</span>
        </div>
        <div class="detail-item full-width">
          <label>文件列表:</label>
          <ul class="file-list">
            <li v-for="file in store.componentInfo.files" :key="file">
              {{ file }}
            </li>
          </ul>
        </div>
      </div>
    </div>

    <div v-if="store.isLoading" class="loading-overlay">
      <div class="spinner"></div>
      <p>处理中...</p>
    </div>
  </div>
</template>

<style scoped>
.components-view {
  position: relative;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.header h2 {
  color: var(--primary-color);
}

.refresh-btn {
  padding: 0.5rem 1rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.refresh-btn:hover {
  background: #357abd;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 8px;
}

.empty-state p {
  color: var(--secondary-color);
}

.empty-state .hint {
  font-size: 0.875rem;
  margin-top: 0.5rem;
}

.components-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.component-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: white;
  border-radius: 8px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.component-card:hover {
  border-color: var(--primary-color);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.component-card.selected {
  border-color: var(--success-color);
  background: #f0fff4;
}

.component-icon {
  width: 48px;
  height: 48px;
  color: var(--primary-color);
}

.component-icon svg {
  width: 100%;
  height: 100%;
}

.component-info {
  flex: 1;
}

.component-info h3 {
  font-size: 1rem;
  margin-bottom: 0.25rem;
}

.component-info .size {
  font-size: 0.875rem;
  color: var(--secondary-color);
}

.component-status {
  color: var(--success-color);
  font-size: 1.5rem;
}

.component-details {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.component-details h3 {
  color: var(--primary-color);
  margin-bottom: 1rem;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.detail-item.full-width {
  grid-column: 1 / -1;
}

.detail-item label {
  font-weight: bold;
  color: var(--secondary-color);
  font-size: 0.875rem;
}

.file-list {
  max-height: 150px;
  overflow-y: auto;
  padding-left: 1.5rem;
  font-size: 0.875rem;
  color: var(--secondary-color);
}

.file-list li {
  margin: 0.25rem 0;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
