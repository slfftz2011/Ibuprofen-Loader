<script setup lang="ts">
import { useAppStore } from "../stores/app";

const store = useAppStore();
</script>

<template>
  <div class="inject-view">
    <h2>开始注入</h2>

    <div v-if="!store.hasGamePath" class="warning-card">
      <p>未找到游戏目录，无法进行注入</p>
    </div>

    <div v-else-if="!store.selectedComponent" class="info-card">
      <p>请先在"组件管理"页面选择要注入的组件</p>
      <button class="btn primary" @click="store.setView('components')">
        前往选择组件
      </button>
    </div>

    <div v-else class="inject-content">
      <div class="selected-component">
        <h3>已选组件</h3>
        <div class="component-summary">
          <div class="icon">📦</div>
          <div class="info">
            <p class="name">{{ store.componentInfo?.name || store.selectedComponent.name }}</p>
            <p class="version" v-if="store.componentInfo">
              版本: {{ store.componentInfo.version }} | 作者: {{ store.componentInfo.author }}
            </p>
          </div>
        </div>
      </div>

      <div class="inject-steps">
        <h3>注入步骤</h3>
        <div class="steps">
          <div class="step" :class="{ completed: store.injectionProgress >= 20 }">
            <div class="step-number">1</div>
            <div class="step-content">
              <p class="step-title">备份现有文件</p>
              <p class="step-desc">自动备份mods、config、resourcepacks</p>
            </div>
          </div>

          <div class="step" :class="{ completed: store.injectionProgress >= 40, active: store.injectionProgress >= 20 && store.injectionProgress < 40 }">
            <div class="step-number">2</div>
            <div class="step-content">
              <p class="step-title">等待游戏启动</p>
              <p class="step-desc">请启动网易我的世界游戏</p>
            </div>
          </div>

          <div class="step" :class="{ completed: store.injectionProgress >= 60, active: store.injectionProgress >= 40 && store.injectionProgress < 60 }">
            <div class="step-number">3</div>
            <div class="step-content">
              <p class="step-title">注入组件</p>
              <p class="step-desc">复制文件到游戏目录</p>
            </div>
          </div>

          <div class="step" :class="{ completed: store.injectionProgress >= 100 }">
            <div class="step-number">4</div>
            <div class="step-content">
              <p class="step-title">完成</p>
              <p class="step-desc">组件注入成功</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="store.isInjecting" class="progress-section">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: store.injectionProgress + '%' }"></div>
        </div>
        <p class="progress-message">{{ store.injectionMessage }}</p>
      </div>

      <div v-if="store.injectionProgress >= 100" class="success-card">
        <p>✅ 注入完成！组件已成功注入到游戏目录</p>
      </div>

      <div v-if="store.error" class="error-card">
        <p>❌ {{ store.error }}</p>
      </div>

      <div class="action-buttons">
        <button 
          class="btn primary large" 
          @click="store.startInjection"
          :disabled="store.isInjecting"
        >
          {{ store.isInjecting ? '注入中...' : '开始注入' }}
        </button>
        <button 
          class="btn" 
          @click="store.clearSelection"
          :disabled="store.isInjecting"
        >
          重新选择组件
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.inject-view h2 {
  color: var(--primary-color);
  margin-bottom: 1.5rem;
}

.warning-card, .info-card {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.warning-card {
  border-left: 4px solid var(--danger-color);
  background: #fff3f3;
}

.info-card {
  border-left: 4px solid var(--primary-color);
}

.inject-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.selected-component {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.selected-component h3 {
  color: var(--primary-color);
  margin-bottom: 1rem;
}

.component-summary {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.component-summary .icon {
  font-size: 2rem;
}

.component-summary .info .name {
  font-weight: bold;
  font-size: 1.1rem;
}

.component-summary .info .version {
  font-size: 0.875rem;
  color: var(--secondary-color);
}

.inject-steps {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.inject-steps h3 {
  color: var(--primary-color);
  margin-bottom: 1rem;
}

.steps {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.step {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-color);
  border-radius: 6px;
  opacity: 0.6;
  transition: all 0.3s;
}

.step.completed {
  opacity: 1;
  background: #f0fff4;
}

.step.active {
  opacity: 1;
  background: #e6f7ff;
  border-left: 3px solid var(--primary-color);
}

.step-number {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--secondary-color);
  color: white;
  border-radius: 50%;
  font-weight: bold;
  flex-shrink: 0;
}

.step.completed .step-number {
  background: var(--success-color);
}

.step.active .step-number {
  background: var(--primary-color);
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.step-content .step-title {
  font-weight: bold;
  margin-bottom: 0.25rem;
}

.step-content .step-desc {
  font-size: 0.875rem;
  color: var(--secondary-color);
}

.progress-section {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.progress-bar {
  height: 8px;
  background: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color), var(--success-color));
  transition: width 0.3s ease;
}

.progress-message {
  text-align: center;
  color: var(--secondary-color);
}

.success-card {
  background: #d4edda;
  border: 1px solid var(--success-color);
  border-radius: 8px;
  padding: 1rem;
  text-align: center;
  color: #155724;
}

.error-card {
  background: #f8d7da;
  border: 1px solid var(--danger-color);
  border-radius: 8px;
  padding: 1rem;
  text-align: center;
  color: #721c24;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s;
  background: var(--bg-color);
  color: var(--text-color);
}

.btn:hover:not(:disabled) {
  background: var(--border-color);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.primary {
  background: var(--primary-color);
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #357abd;
}

.btn.large {
  padding: 1rem 2rem;
  font-size: 1.1rem;
}
</style>
