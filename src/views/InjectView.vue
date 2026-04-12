<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../stores/app";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import SLProgress from "../components/common/SLProgress.vue";

const router = useRouter();
const store = useAppStore();

const isRunning = ref(false);
const cancelRequested = ref(false);

onMounted(() => {
  if (!store.extractedComponent || !store.gamePath) {
    router.push('/components');
  }
});

const canInject = computed(() => 
  !!store.gamePath && !!store.extractedComponent && !!store.componentInfo && !store.isInjecting
);

async function startInjectProcess() {
  if (!canInject.value) return;
  
  isRunning.value = true;
  cancelRequested.value = false;
  
  try {
    await store.startInjection();
  } finally {
    isRunning.value = false;
  }
}

function requestCancel() {
  cancelRequested.value = true;
  // Note: actual cancel logic would need backend support
}

function goBack() {
  router.push('/components');
}
</script>

<template>
  <div class="inject-view">
    <SLCard title="模组注入" subtitle="自动备份 + 等待游戏启动 + 文件注入">
      <div v-if="!store.componentInfo" class="loading-state">
        <div class="spinner"></div>
        <span>加载组件信息...</span>
      </div>

      <div v-else class="inject-content">
        <!-- Component Info -->
        <div class="component-summary">
          <h2>{{ store.componentInfo.name }}</h2>
          <p class="component-version">版本 {{ store.componentInfo.version }} by {{ store.componentInfo.author }}</p>
          <p class="component-desc">{{ store.componentInfo.description }}</p>
          <div class="component-stats">
            <span>文件: {{ store.componentInfo.files.length }} 个</span>
            <span>游戏路径: {{ store.gamePath || '未配置' }}</span>
          </div>
        </div>

        <!-- Progress -->
        <div class="progress-section" v-if="store.isInjecting">
          <SLProgress :value="store.injectionProgress" size="lg" />
          <div class="progress-text">
            {{ store.injectionMessage }}
          </div>
          <div class="progress-time">
            预计 {{ store.injectionProgress < 40 ? '2-3分钟' : '30秒内' }} 完成
          </div>
          <SLButton variant="danger" @click="requestCancel" :disabled="!cancelRequested">
            取消注入 (谨慎)
          </SLButton>
        </div>

        <!-- Control Buttons -->
        <div v-else class="control-buttons">
          <SLButton 
            variant="primary" 
            size="lg" 
            @click="startInjectProcess"
            :disabled="!canInject"
            class="inject-btn"
          >
            🚀 开始注入
          </SLButton>
          <div class="button-group">
            <SLButton variant="ghost" @click="goBack">
              ← 返回组件列表
            </SLButton>
            <SLButton variant="secondary" @click="router.push('/settings')">
              检查设置
            </SLButton>
          </div>
        </div>

        <!-- Injection Steps Preview -->
        <div class="steps-preview">
          <h4>注入流程</h4>
          <div class="steps-list">
            <div class="step">
              <div class="step-number">1</div>
              <span>备份 mods/config/resourcepacks → game/backup</span>
            </div>
            <div class="step">
              <div class="step-number">2</div>
              <span>等待游戏启动 (游戏删除 trigger.txt)</span>
            </div>
            <div class="step">
              <div class="step-number">3</div>
              <span>注入组件文件到游戏目录</span>
            </div>
          </div>
        </div>
      </div>
    </SLCard>
  </div>
</template>

<style scoped>
.inject-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-2xl);
  max-width: 800px;
}

.component-summary {
  text-align: center;
  padding: var(--sl-space-2xl);
  background: linear-gradient(135deg, rgba(var(--sl-primary-rgb), 0.1), rgba(var(--sl-secondary-rgb), 0.05));
  border-radius: var(--sl-radius-xl);
  border: 1px solid var(--sl-border-light);
}

.component-summary h2 {
  margin: 0 0 var(--sl-space-sm) 0;
  font-size: 1.75rem;
  font-weight: 700;
  background: linear-gradient(var(--sl-primary), var(--sl-secondary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.component-version {
  color: var(--sl-success);
  font-weight: 600;
  margin: 0 0 var(--sl-space-xs) 0;
}

.component-desc {
  color: var(--sl-text-secondary);
  font-size: 1rem;
  line-height: 1.6;
  margin: 0 0 var(--sl-space-md) 0;
  max-width: 600px;
}

.component-stats {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sl-space-md);
  justify-content: center;
  font-size: 0.875rem;
  color: var(--sl-text-tertiary);
}

.progress-section {
  text-align: center;
  padding: var(--sl-space-xl);
}

.progress-text {
  font-size: 1.125rem;
  font-weight: 500;
  margin: var(--sl-space-md) 0;
  min-height: 1.5em;
}

.progress-time {
  color: var(--sl-text-secondary);
  font-size: 0.875rem;
  margin-bottom: var(--sl-space-lg);
}

.control-buttons {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--sl-space-xl);
}

.inject-btn {
  width: 280px;
  height: 56px;
}

.button-group {
  display: flex;
  gap: var(--sl-space-lg);
}

.steps-preview {
  opacity: 0.8;
}

.steps-preview h4 {
  margin-bottom: var(--sl-space-md);
  color: var(--sl-text-primary);
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.step {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
  background: rgba(255,255,255,0.03);
  border-radius: var(--sl-radius-md);
  border-left: 4px solid var(--sl-primary);
}

.step-number {
  width: 32px;
  height: 32px;
  background: var(--sl-primary);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.875rem;
  flex-shrink: 0;
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--sl-space-md);
  padding: var(--sl-space-2xl);
  color: var(--sl-text-tertiary);
}

.spinner {
  width: 24px;
  height: 24px;
  border: 3px solid var(--sl-border);
  border-top-color: var(--sl-primary);
  border-radius: 50%;
  animation: sl-spin 0.8s linear infinite;
}

@media (max-width: 600px) {
  .button-group {
    flex-direction: column;
    align-items: center;
  }
}
</style>

