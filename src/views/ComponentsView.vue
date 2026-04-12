<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../stores/app";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import SLInput from "../components/common/SLInput.vue";
import type { CopFileInfo, ListJsonInfo } from "../types";

const router = useRouter();
const store = useAppStore();

const selectedCop = ref<CopFileInfo | null>(null);
const previewInfo = ref<ListJsonInfo | null>(null);
const searchQuery = ref("");

const filteredComponents = computed(() => 
  store.components.filter(c => 
    c.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
);

const humanSize = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

async function handleSelectCop(cop: CopFileInfo) {
  selectedCop.value = cop;
  store.selectComponent(cop);
  previewInfo.value = store.componentInfo;
}

function clearSelection() {
  selectedCop.value = null;
  previewInfo.value = null;
  store.clearSelection();
}

onMounted(() => {
  if (store.components.length === 0) {
    store.scanComponents();
  }
});

</script>

<template>
  <div class="components-view">
    <!-- Search & Stats -->
    <div class="header-row">
      <SLInput 
        v-model="searchQuery" 
        placeholder="搜索组件..."
        class="search-input"
      />
      <div class="stats">
        <span>{{ filteredComponents.length }} / {{ store.components.length }} 个组件</span>
        <SLButton @click="store.scanComponents()" variant="ghost">
          刷新
        </SLButton>
      </div>
    </div>

    <!-- Components Grid -->
    <div v-if="store.components.length === 0" class="empty-state">
      <svg width="80" height="80" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
        <polyline points="17 8 12 13 7 8"/>
        <path d="M7 20v-5a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v5"/>
        <line x1="7" y1="3" x2="17" y2="3"/>
        <line x1="12" y1="12" x2="12" y2="20"/>
      </svg>
      <h3>还没有组件</h3>
      <p>点击刷新扫描组件目录，或将 .cop 文件放入目录</p>
      <SLButton @click="router.push('/settings')" variant="primary">
        配置目录
      </SLButton>
    </div>

    <div v-else class="components-grid">
      <div 
        v-for="cop in filteredComponents" 
        :key="cop.path"
        class="cop-card"
        :class="{ selected: selectedCop?.path === cop.path }"
        @click="handleSelectCop(cop)"
      >
        <div class="cop-header">
          <h4 class="cop-name">{{ cop.name }}</h4>
          <span class="cop-size">{{ humanSize(cop.size) }}</span>
        </div>
        <div class="cop-path">{{ cop.path.split('\\').pop()?.split('/').pop() || cop.path }}</div>
        <div class="cop-actions">
          <SLButton size="sm" variant="ghost">预览</SLButton>
        </div>
      </div>
    </div>

    <!-- Component Preview -->
    <SLCard v-if="previewInfo" title="组件预览" class="preview-card">
      <div class="preview-content">
        <div class="preview-meta">
          <h3>{{ previewInfo.name }}</h3>
          <div class="meta-tags">
            <span class="tag version">v{{ previewInfo.version }}</span>
            <span class="tag author">作者: {{ previewInfo.author }}</span>
          </div>
          <p class="preview-desc">{{ previewInfo.description }}</p>
        </div>
        <div class="preview-files">
          <h4>文件列表 ({{ previewInfo.files.length }} 个)</h4>
          <div class="files-list">
            <div v-for="(file, i) in previewInfo.files.slice(0, 10)" :key="i" class="file-item">
              {{ file }}
            </div>
            <span v-if="previewInfo.files.length > 10" class="more-files">
              ... 还有 {{ previewInfo.files.length - 10 }} 个文件
            </span>
          </div>
        </div>
        <div class="preview-actions">
          <SLButton 
            variant="primary" 
            @click="router.push('/inject')"
            :disabled="!store.hasGamePath || store.isLoading"
            :loading="store.isLoading"
          >
            使用此组件注入
          </SLButton>
          <SLButton variant="ghost" @click="clearSelection()">
            取消选择
          </SLButton>
        </div>
      </div>
    </SLCard>

    <!-- Loading -->
    <div v-if="store.isLoading" class="loading-state">
      <div class="spinner"></div>
      <span>处理组件...</span>
    </div>
  </div>
</template>

<style scoped>
.components-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-lg);
}

.header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--sl-space-md);
}

.search-input {
  flex: 1;
  max-width: 400px;
}

.stats {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  color: var(--sl-text-secondary);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--sl-space-3xl);
  gap: var(--sl-space-lg);
  color: var(--sl-text-tertiary);
}

.empty-state svg {
  color: var(--sl-text-tertiary);
}

.empty-state h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
}

.empty-state p {
  margin: 0;
  text-align: center;
  opacity: 0.8;
}

.components-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--sl-space-md);
}

.cop-card {
  padding: var(--sl-space-lg);
  border: 2px solid var(--sl-border-light);
  border-radius: var(--sl-radius-lg);
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--sl-surface);
}

.cop-card:hover {
  border-color: var(--sl-primary);
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
}

.cop-card.selected {
  border-color: var(--sl-primary);
  background: var(--sl-primary-bg);
  box-shadow: 0 0 0 3px rgba(var(--sl-primary-rgb), 0.1);
}

.cop-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--sl-space-sm);
}

.cop-name {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0;
  flex: 1;
  line-height: 1.3;
}

.cop-size {
  font-family: var(--sl-font-mono);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
  white-space: nowrap;
}

.cop-path {
  font-family: var(--sl-font-mono);
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
  word-break: break-all;
  margin-bottom: var(--sl-space-md);
}

.preview-card {
  margin-top: var(--sl-space-xl);
}

.preview-content {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-lg);
}

.preview-meta {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-sm);
}

.meta-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sl-space-xs);
}

.tag {
  padding: 2px 8px;
  border-radius: var(--sl-radius-full);
  font-size: 0.75rem;
  font-weight: 500;
}

.tag.version {
  background: var(--sl-success-bg);
  color: var(--sl-success);
}

.tag.author {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.preview-desc {
  color: var(--sl-text-secondary);
  line-height: 1.6;
}

.files-list {
  max-height: 200px;
  overflow-y: auto;
  background: rgba(255,255,255,0.02);
  border-radius: var(--sl-radius-sm);
  padding: var(--sl-space-sm);
}

.file-item {
  font-family: var(--sl-font-mono);
  font-size: 0.75rem;
  padding: 2px 0;
  opacity: 0.8;
}

.more-files {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
  font-style: italic;
}

.preview-actions {
  display: flex;
  gap: var(--sl-space-md);
  padding-top: var(--sl-space-md);
  border-top: 1px solid var(--sl-border-light);
}

.loading-state {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  justify-content: center;
  padding: var(--sl-space-xl);
  color: var(--sl-text-tertiary);
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--sl-border);
  border-top-color: var(--sl-primary);
  border-radius: 50%;
  animation: sl-spin 0.8s linear infinite;
}

@media (max-width: 768px) {
  .header-row {
    flex-direction: column;
    align-items: stretch;
  }
  
  .preview-actions {
    flex-direction: column;
  }
}
</style>

