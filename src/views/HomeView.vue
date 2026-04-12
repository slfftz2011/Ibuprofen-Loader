<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../stores/app";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import SLBadge from "../components/common/SLBadge.vue";
import SLProgress from "../components/common/SLProgress.vue";
import * as api from "../api/tauri";
import type { GamePathResult } from "../types";

const router = useRouter();
const store = useAppStore();

const gamePathStatus = ref<GamePathResult | null>(null);
const isInitializing = ref(true);
const actionError = ref<string | null>(null);

onMounted(async () => {
  await initializeDashboard();
  store.startNetworkPolling();
  isInitializing.value = false;
});

async function initializeDashboard() {
  try {
    gamePathStatus.value = await api.getGamePath();
    await store.scanComponents();
  } catch (e) {
    actionError.value = "初始化失败: " + (e as Error).message;
  }
}

function getPathStatusVariant(): "success" | "warning" | "error" {
  if (!gamePathStatus.value) return "warning";
  if (gamePathStatus.value.path) return "success";
  if (gamePathStatus.value.error) return "error";
  return "warning";
}

function getNetworkVariant(): "success" | "warning" | "error" {
  if (!store.networkStatus) return "warning";
  switch (store.networkStatus.status) {
    case "connected": return "success";
    case "limited": return "warning";
    case "disconnected": return "error";
    default: return "warning";
  }
}
</script>

<template>
  <div class="home-view animate-fade-in-up">
    <!-- Error Banner -->
    <div v-if="actionError" class="error-banner">
      <span>{{ actionError }}</span>
      <button class="error-close" @click="actionError = null">×</button>
    </div>

    <!-- Loader Status Grid -->
    <div class="status-grid">
      <!-- Game Path Status -->
      <SLCard title="游戏路径" class="status-card">
        <div class="status-main">
          <SLBadge 
            :text="gamePathStatus?.path ? '已找到' : '未找到'" 
            :variant="getPathStatusVariant()"
            size="lg"
          />
          <div class="status-path" v-if="gamePathStatus?.path">
            {{ gamePathStatus.path }}
          </div>
          <div class="status-error" v-else-if="gamePathStatus?.error">
            {{ gamePathStatus.error }}
          </div>
        </div>
        <SLButton 
          variant="ghost" 
          size="sm" 
          @click="initializeDashboard"
          :disabled="isInitializing"
        >
          刷新
        </SLButton>
      </SLCard>

      <!-- Network Status -->
      <SLCard title="网络状态" class="status-card">
        <div class="status-main">
          <SLBadge 
            :text="store.networkStatus?.status === 'connected' ? '已连接' : store.networkStatus?.message || '检测中...'" 
            :variant="getNetworkVariant()"
            size="lg"
          />
          <div class="status-detail">
            GitHub: <SLBadge 
              :text="store.githubConnected ? '✓ 可达' : '✗ 不可达'" 
              :variant="store.githubConnected ? 'success' : 'error'"
              size="sm"
            />
          </div>
        </div>
      </SLCard>

      <!-- Components Status -->
      <SLCard title="组件库" class="status-card">
        <div class="status-main">
          <div class="components-count">
            {{ store.components.length }} 个组件
          </div>
          <SLProgress :value="Math.min(store.components.length * 5, 100)" />
        </div>
        <SLButton 
          variant="primary" 
          @click="router.push('/components')"
          :disabled="!store.hasGamePath"
        >
          管理组件
        </SLButton>
      </SLCard>

      <!-- Quick Actions -->
      <SLCard title="快速操作">
        <div class="quick-actions">
          <SLButton 
            variant="primary" 
            size="lg" 
            @click="router.push(store.hasComponents ? '/inject' : '/components')"
            :disabled="!store.isNetworkConnected || !store.hasGamePath"
          >
            一键注入
          </SLButton>
          <SLButton variant="secondary" size="lg" @click="router.push('/settings')">
            设置
          </SLButton>
        </div>
      </SLCard>
    </div>

    <!-- Recent Activity / Tips -->
    <SLCard title="准备工作">
      <div class="tips-grid">
        <div class="tip-item">
          <div class="tip-icon">📁</div>
          <div>
            <h4>组件目录</h4>
            <p>将 .cop 文件放入组件目录，点击“管理组件”扫描</p>
          </div>
        </div>
        <div class="tip-item">
          <div class="tip-icon">🎮</div>
          <div>
            <h4>启动游戏</h4>
            <p>点击“一键注入”后启动网易我的世界，自动注入组件</p>
          </div>
        </div>
        <div class="tip-item">
          <div class="tip-icon">🔄</div>
          <div>
            <h4>自动备份</h4>
            <p>原 mods/config/resourcepacks 会自动备份到游戏目录/backup</p>
          </div>
        </div>
      </div>
    </SLCard>

    <!-- Loading Overlay -->
    <div v-if="isInitializing" class="loading-overlay">
      <div class="spinner-lg"></div>
      <span>初始化仪表盘...</span>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-lg);
  padding-bottom: var(--sl-space-xl);
}

.error-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sl-space-sm) var(--sl-space-md);
  background: rgba(var(--sl-error-rgb), 0.1);
  border: 1px solid rgba(var(--sl-error-rgb), 0.2);
  border-radius: var(--sl-radius-md);
  color: var(--sl-error);
  font-size: 0.875rem;
  backdrop-filter: blur(10px);
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--sl-space-md);
}

.status-card {
  display: flex;
  flex-direction: column;
}

.status-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-sm);
  margin-bottom: var(--sl-space-md);
}

.status-path {
  font-family: var(--sl-font-mono);
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
  word-break: break-all;
  padding: var(--sl-space-xs) var(--sl-space-sm);
  background: rgba(255,255,255,0.05);
  border-radius: var(--sl-radius-sm);
}

.status-error {
  color: var(--sl-error);
  font-size: 0.875rem;
}

.components-count {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--sl-primary);
}

.quick-actions {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.tips-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--sl-space-lg);
}

.tip-item {
  display: flex;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
  border: 1px solid var(--sl-border-light);
  border-radius: var(--sl-radius-md);
  transition: all 0.2s ease;
}

.tip-item:hover {
  border-color: var(--sl-primary);
  background: rgba(var(--sl-primary-rgb), 0.05);
}

.tip-icon {
  font-size: 2rem;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--sl-radius-md);
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
  flex-shrink: 0;
}

.tip-item h4 {
  margin: 0 0 var(--sl-space-xs) 0;
  font-size: 0.9375rem;
  font-weight: 600;
}

.tip-item p {
  margin: 0;
  color: var(--sl-text-secondary);
  font-size: 0.875rem;
  line-height: 1.5;
}

.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.spinner-lg {
  width: 48px;
  height: 48px;
  border: 4px solid var(--sl-border);
  border-top-color: var(--sl-primary);
  border-radius: 50%;
  animation: sl-spin 1s linear infinite;
  margin-bottom: var(--sl-space-sm);
}

@media (max-width: 768px) {
  .status-grid {
    grid-template-columns: 1fr;
  }
  
  .tips-grid {
    grid-template-columns: 1fr;
  }
}
</style>

