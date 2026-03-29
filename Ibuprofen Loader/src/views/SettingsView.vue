<script setup lang="ts">
import { useAppStore } from "../stores/app";
import { useToast } from "../composables/useToast";
import * as api from "../api/commands";

const store = useAppStore();
const { showInfoToast } = useToast();

const githubUrl = "https://github.com/slfftz2011";
const wattToolkitUrl = "https://steampp.net/";

async function handleTestGithub(retry = 0) {
  const { showSuccessToast, showErrorToast, showWarningToast } = useToast();
  showInfoToast(`测试GitHub连通性... (第${retry + 1}次)`);
  await store.testGithubConnectivity();
  if (store.githubStatus?.reachable === true) {
    showSuccessToast('✓ GitHub连通正常');
  } else if (retry < 2) {
    showWarningToast(`测试失败，重试中... (${retry + 1}/3)`);
    setTimeout(() => handleTestGithub(retry + 1), 1000);
  } else {
    showErrorToast(store.githubStatus?.error || '✗ GitHub连通异常 (3次重试失败)');
  }
}

async function handleTestNetwork() {
  const { showInfoToast, showSuccessToast, showErrorToast } = useToast();
  const isTesting = true;
  showInfoToast('刷新网络状态...');
  try {
    store.networkStatus = await api.checkConnectionStatus();
    showSuccessToast('网络状态已刷新');
  } catch (e) {
    showErrorToast('刷新失败');
  }
}

</script>

<template>
  <div class="settings-view">
    <h2>设置</h2>

    <div class="settings-section">
      <h3>游戏配置</h3>
      <div class="setting-item">
        <label>游戏路径</label>
        <div class="setting-value">
          <span v-if="store.gamePath">{{ store.gamePath }}</span>
          <span v-else class="not-found">未找到</span>
        </div>
      </div>
      <div class="setting-item">
        <label>组件目录</label>
        <div class="setting-value">
          <span>{{ store.components.length > 0 ? store.components[0]?.path.replace(/\\[^\\]+\\.cop$/i, 'components') : '默认目录' }}</span>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h3>网络设置</h3>
      <div class="setting-item">
        <label>网络状态</label>
        <div class="setting-value">
        <button class="btn small ml-2" @click="(e) => { e.stopPropagation(); handleTestNetwork(); }">
            刷新
          </button>
          <span :class="['status-badge', store.networkStatus?.status]">
            {{ store.networkStatus?.status === 'connected' ? '已连接' : 
               store.networkStatus?.status === 'limited_access' ? '受限' : '未连接' }}
          </span>
        </div>
      </div>
      <div class="setting-item">
        <label>测试连接</label>
        <div class="setting-actions">

        <button class="btn small" @click="(e) => { e.stopPropagation(); handleTestGithub(); }">
            测试 GitHub
          </button>
          <span class="status-badge ml-2" :class="store.githubStatus?.reachable === true ? 'connected' : 'disconnected'">
            {{ store.githubStatus?.reachable === true ? '✓正常' : store.githubStatus?.error || '未测试' }}
          </span>

        </div>
      </div>
    </div>


    <div class="settings-section">
      <h3>快捷链接</h3>
      <div class="links-grid">

        <button class="link-card" @click="store.openExternalUrl(githubUrl)">
          <span class="link-icon">🔗</span>
          <span class="link-text">GitHub</span>
        </button>

        <button class="link-card" @click="store.openExternalUrl(wattToolkitUrl)">
          <span class="link-icon">⚡</span>
          <span class="link-text">Watt Toolkit</span>
        </button>
      </div>
    </div>

    <div class="settings-section">
      <h3>关于</h3>
      <div class="about-info">
        <p><strong>网易MC组件注入器</strong></p>
        <p>版本: 3.0.0</p>
        <p>技术栈: Rust + Tauri + Vue 3 + TypeScript</p>
        <p class="copyright">基于 NeteaseModInjector2 重构</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-view h2 {
  color: var(--primary-color);
  margin-bottom: 1.5rem;
}

.settings-section {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 1.5rem;
}

.settings-section h3 {
  color: var(--primary-color);
  margin-bottom: 1rem;
  font-size: 1.1rem;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.5rem;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item label {
  font-weight: 500;
  color: var(--text-color);
}

.setting-value {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.setting-value span {
  color: var(--secondary-color);
}

.setting-value .not-found {
  color: var(--danger-color);
}

.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 0.875rem;
}

.status-badge.connected {
  background: #d4edda;
  color: #155724;
}

.status-badge.disconnected {
  background: #f8d7da;
  color: #721c24;
}

.status-badge.limited_access {
  background: #fff3cd;
  color: #856404;
}

.setting-actions {
  display: flex;
  gap: 0.5rem;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  background: var(--primary-color);
  color: white;
  transition: all 0.3s;
}

.btn:hover {
  background: #357abd;
}

.btn.small {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
}

.links-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
}

.link-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  background: var(--bg-color);
  border: 2px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.link-card:hover {
  border-color: var(--primary-color);
  transform: translateY(-2px);
}

.link-icon {
  font-size: 1.5rem;
}

.link-text {
  font-weight: 500;
}

.about-info {
  text-align: center;
}

.about-info p {
  margin: 0.5rem 0;
}

.about-info .copyright {
  color: var(--secondary-color);
  font-size: 0.875rem;
  margin-top: 1rem;
}
</style>

