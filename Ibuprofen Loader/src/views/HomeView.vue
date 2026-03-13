<script setup lang="ts">
import { useAppStore } from "../stores/app";

const store = useAppStore();
</script>

<template>
  <div class="home-view">
    <div class="welcome-card">
      <h2>欢迎使用Ibuprofen Loader</h2>
      <p class="version">版本 3.0.0</p>
    </div>

    <div class="status-cards">
      <div class="status-card">
        <h3>游戏状态</h3>
        <div class="status-content">
          <span :class="['indicator', store.hasGamePath ? 'success' : 'error']"></span>
          <p v-if="store.hasGamePath">
            已找到游戏目录<br>
            <small>{{ store.gamePath }}</small>
          </p>
          <p v-else>
            未找到网易我的世界安装目录<br>
            <small>请确保已安装网易我的世界启动器</small>
          </p>
        </div>
      </div>

      <div class="status-card">
        <h3>网络状态</h3>
        <div class="status-content">
          <span :class="['indicator', store.isNetworkConnected ? 'success' : 'error']"></span>
          <p v-if="store.networkStatus">
            {{ store.networkStatus.status === 'connected' ? '已连接互联网' : 
               store.networkStatus.status === 'limited_access' ? '网络受限' : '未连接网络' }}
          </p>
        </div>
      </div>

      <div class="status-card">
        <h3>组件数量</h3>
        <div class="status-content">
          <span class="count">{{ store.components.length }}</span>
          <p>个可用组件</p>
        </div>
      </div>
    </div>

    <div class="quick-actions">
      <h3>快速操作</h3>
      <div class="action-buttons">
        <button 
          class="action-btn primary" 
          @click="store.setView('components')"
          :disabled="!store.hasGamePath"
        >
          管理组件
        </button>
        <button 
          class="action-btn success" 
          @click="store.setView('inject')"
          :disabled="!store.hasComponents"
        >
          开始注入
        </button>
        <button 
          class="action-btn" 
          @click="store.setView('settings')"
        >
          设置
        </button>
      </div>
    </div>

    <div class="info-section" v-if="!store.hasGamePath">
      <h3>帮助信息</h3>
      <p>未检测到网易我的世界安装。请确保：</p>
      <ul>
        <li>已安装网易我的世界启动器</li>
        <li>启动器已正常运行过一次</li>
        <li>如有问题，请尝试重新安装启动器</li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.welcome-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem;
  border-radius: 12px;
  text-align: center;
}

.welcome-card h2 {
  margin-bottom: 0.5rem;
}

.welcome-card .version {
  opacity: 0.8;
}

.status-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.status-card {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.status-card h3 {
  color: var(--primary-color);
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

.status-content {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

.indicator.success {
  background: var(--success-color);
  box-shadow: 0 0 8px var(--success-color);
}

.indicator.error {
  background: var(--danger-color);
  box-shadow: 0 0 8px var(--danger-color);
}

.status-content p {
  margin: 0;
}

.status-content small {
  color: var(--secondary-color);
  font-size: 0.875rem;
}

.count {
  font-size: 2rem;
  font-weight: bold;
  color: var(--primary-color);
}

.quick-actions {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.quick-actions h3 {
  margin-bottom: 1rem;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.action-btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s;
  background: var(--bg-color);
  color: var(--text-color);
}

.action-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.primary {
  background: var(--primary-color);
  color: white;
}

.action-btn.success {
  background: var(--success-color);
  color: white;
}

.info-section {
  background: #fff3cd;
  padding: 1.5rem;
  border-radius: 8px;
  border-left: 4px solid var(--warning-color);
}

.info-section h3 {
  color: #856404;
  margin-bottom: 0.5rem;
}

.info-section ul {
  margin-top: 0.5rem;
  padding-left: 1.5rem;
}

.info-section li {
  margin: 0.25rem 0;
}
</style>
