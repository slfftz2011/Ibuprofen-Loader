<script setup lang="ts">
import { onMounted } from "vue";
import { useAppStore } from "./stores/app";
import * as api from "./api/commands";
import HomeView from "./views/HomeView.vue";
import ComponentsView from "./views/ComponentsView.vue";
import InjectView from "./views/InjectView.vue";
import SettingsView from "./views/SettingsView.vue";
import ToastContainer from "./components/ToastContainer.vue";

const store = useAppStore();

onMounted(() => {
  store.initialize();
  // 1s后开始3种检查
  setTimeout(async () => {
    try {
      const pathResult = await api.getNeteaseDownloadPath();
      if (pathResult.path) store.gamePath = pathResult.path;
    } catch (e) {}
    
    try {
      store.components = await api.scanCopFiles();
      await api.checkOrCreateComponentsDir();
    } catch (e) {}
    
    store.checkNetworkBackground();
  }, 1000);
});
</script>

<template>
  <div class="app">
    <header class="app-header">
      <h1>网易MC组件注入器</h1>
      <div class="status-bar">
        <span :class="['status-item', store.networkStatus?.status]">
          网络: {{ store.networkStatus?.status === 'checking' ? '检测中...' : store.networkStatus?.status === 'connected' ? '已连接' : store.networkStatus?.status === 'limited_access' ? '受限' : '未连接' }}
        </span>
        <span class="status-item" v-if="store.gamePath">
          游戏路径: {{ store.gamePath }}
        </span>
      </div>
    </header>

    <nav class="app-nav">
      <button 
        :class="{ active: store.currentView === 'home' }" 
        @click="store.setView('home')"
      >
        首页
      </button>
      <button 
        :class="{ active: store.currentView === 'components' }" 
        @click="store.setView('components')"
      >
        组件管理
      </button>
      <button 
        :class="{ active: store.currentView === 'inject' }" 
        @click="store.setView('inject')"
      >
        开始注入
      </button>
      <button 
        :class="{ active: store.currentView === 'settings' }" 
        @click="store.setView('settings')"
      >
        设置
      </button>
    </nav>

    <main class="app-content">
      <div v-if="store.isLoading" class="loading">
        <div class="spinner"></div>
        <p>加载中...</p>
      </div>

      <div v-else-if="store.error && store.currentView !== 'settings'" class="error-message">
        <p>{{ store.error }}</p>
        <button @click="store.clearError">关闭</button>
      </div>

      <HomeView v-else-if="store.currentView === 'home'" />
      <ComponentsView v-else-if="store.currentView === 'components'" />
      <InjectView v-else-if="store.currentView === 'inject'" />
      <SettingsView v-else-if="store.currentView === 'settings'" />
      </main>
      <ToastContainer />
    <footer class="app-footer">
<p>Ibuprofen Loader v3.0.0 - Powered by Rust + Vue</p>
    </footer>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --primary-color: #4a90d9;
  --secondary-color: #6c757d;
  --success-color: #28a745;
  --danger-color: #dc3545;
  --warning-color: #ffc107;
  --bg-color: #f5f5f5;
  --text-color: #333;
  --border-color: #ddd;
}

body {
  font-family: 'Microsoft YaHei', 'Segoe UI', sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
  line-height: 1.6;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1rem 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.app-header h1 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
}

.status-bar {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
}

.status-item {
  padding: 0.25rem 0.75rem;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

.status-item.connected {
  background: rgba(40, 167, 69, 0.3);
}

.status-item.disconnected {
  background: rgba(220, 53, 69, 0.3);
}

.status-item.limited_access {
  background: rgba(255, 193, 7, 0.3);
}

.app-nav {
  display: flex;
  background: white;
  padding: 0.5rem 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.app-nav button {
  padding: 0.75rem 1.5rem;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 1rem;
  color: var(--text-color);
  border-bottom: 3px solid transparent;
  transition: all 0.3s;
}

.app-nav button:hover {
  background: var(--bg-color);
}

.app-nav button.active {
  border-bottom-color: var(--primary-color);
  color: var(--primary-color);
  font-weight: bold;
}

.app-content {
  flex: 1;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.app-footer {
  text-align: center;
  padding: 1rem;
  background: white;
  color: var(--secondary-color);
  font-size: 0.875rem;
  border-top: 1px solid var(--border-color);
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
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

.error-message {
  background: #fff3f3;
  border: 1px solid var(--danger-color);
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
}

.error-message p {
  color: var(--danger-color);
  margin-bottom: 1rem;
}

.error-message button {
  padding: 0.5rem 1.5rem;
  background: var(--danger-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
</style>
