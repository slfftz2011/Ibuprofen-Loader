<script setup lang="ts">
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";

const version = "3.0.0";
const buildDate = new Date().getFullYear();

function openUrl(url: string) {
  if (url) window.open(url, "_blank");
}
</script>

<template>
  <div class="about-view animate-fade-in-up">
    <div class="hero-section">
      <div class="hero-logo">
        <svg width="72" height="72" viewBox="0 0 72 72" fill="none">
          <rect x="4" y="4" width="64" height="64" rx="16" fill="var(--sl-primary)" opacity="0.1" />
          <rect x="12" y="12" width="48" height="48" rx="12" fill="var(--sl-primary)" opacity="0.2" />
          <rect x="22" y="22" width="28" height="28" rx="6" fill="var(--sl-primary)" opacity="0.4" />
          <rect x="28" y="28" width="16" height="16" rx="3" fill="var(--sl-primary)" />
        </svg>
      </div>
      <h1 class="hero-title">Ibuprofen Loader</h1>
      <p class="hero-subtitle">网易我的世界组件注入器</p>
      <div class="hero-badges">
        <span class="version-badge">v{{ version }}</span>
        <span class="tech-badge">Vue 3 + Tauri 2</span>
        <span class="license-badge">MIT</span>
      </div>
      <p class="hero-desc">
        现代化组件注入器，提供自动扫描、备份恢复、游戏路径检测与注入流程控制。
      </p>
    </div>

    <SLCard title="核心功能">
      <div class="feature-grid">
        <div class="feature-item">
          <h3>组件扫描</h3>
          <p>自动识别 .COP 文件并展示组件详情。</p>
        </div>
        <div class="feature-item">
          <h3>自动备份</h3>
          <p>注入前备份 mods/config/resourcepacks，支持安全恢复。</p>
        </div>
        <div class="feature-item">
          <h3>游戏路径检测</h3>
          <p>读取网易 MC 安装路径，并支持手动覆盖。</p>
        </div>
        <div class="feature-item">
          <h3>一键注入</h3>
          <p>等待游戏启动并自动完成文件复制流程。</p>
        </div>
      </div>
    </SLCard>

    <div class="info-grid">
      <SLCard title="项目信息">
        <div class="info-list">
          <div class="info-item"><span>版本</span><span>{{ version }}</span></div>
          <div class="info-item"><span>构建年份</span><span>{{ buildDate }}</span></div>
          <div class="info-item"><span>前端</span><span>Vue 3 + Pinia</span></div>
          <div class="info-item"><span>后端</span><span>Rust + Tauri</span></div>
          <div class="info-item"><span>许可证</span><span>MIT</span></div>
        </div>
      </SLCard>

      <SLCard title="参与方式">
        <div class="contribute-ways">
          <div class="way-item">
            <div class="way-icon">💻</div>
            <div class="way-info">
              <span class="way-title">写代码</span>
              <span class="way-desc">修复 Bug、实现新功能。</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">🎨</div>
            <div class="way-info">
              <span class="way-title">做设计</span>
              <span class="way-desc">优化界面和交互体验。</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">📖</div>
            <div class="way-info">
              <span class="way-title">写文档</span>
              <span class="way-desc">完善使用说明与开发指南。</span>
            </div>
          </div>
        </div>
      </SLCard>
    </div>

    <div class="links-section">
      <SLButton variant="primary" size="lg" @click="openUrl('https://github.com/slfftz2011/Ibuprofen-Loader')">
        GitHub 仓库
      </SLButton>
      <SLButton variant="secondary" size="lg" @click="openUrl('https://github.com/slfftz2011/Ibuprofen-Loader/releases')">
        下载最新版本
      </SLButton>
    </div>

    <div class="about-footer">
      <p>Ibuprofen Loader 旨在让组件注入更加简单、安全、稳定。</p>
    </div>
  </div>
</template>

<style scoped>
.about-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xl);
  max-width: 900px;
  margin: 0 auto;
  padding-bottom: var(--sl-space-2xl);
}

.hero-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--sl-space-2xl) 0;
}

.hero-logo {
  margin-bottom: var(--sl-space-md);
}

.hero-title {
  font-size: 2.5rem;
  font-weight: 800;
  color: var(--sl-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--sl-space-xs);
}

.hero-subtitle {
  font-size: 1.125rem;
  color: var(--sl-text-secondary);
  margin-bottom: var(--sl-space-md);
}

.hero-badges {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sl-space-sm);
  margin-bottom: var(--sl-space-lg);
}

.hero-desc {
  max-width: 640px;
  color: var(--sl-text-secondary);
  line-height: 1.8;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: var(--sl-space-md);
}

.feature-item {
  padding: var(--sl-space-md);
  border-radius: var(--sl-radius-md);
  border: 1px solid var(--sl-border-light);
  background: var(--sl-surface);
}

.feature-item h3 {
  margin-bottom: var(--sl-space-xs);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: var(--sl-space-md);
}

.info-list {
  display: grid;
  gap: var(--sl-space-sm);
}

.info-item {
  display: flex;
  justify-content: space-between;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm) 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.links-section {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sl-space-sm);
  justify-content: center;
}

.about-footer {
  text-align: center;
  color: var(--sl-text-tertiary);
}

.version-badge, .tech-badge, .license-badge {
  padding: 4px 14px;
  border-radius: var(--sl-radius-full);
  font-size: 0.8125rem;
  font-weight: 500;
}

.version-badge {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.tech-badge {
  background: rgba(34, 197, 94, 0.1);
  color: var(--sl-success);
}

.license-badge {
  background: rgba(168, 85, 247, 0.1);
  color: #a855f7;
}

.hero-desc {
  font-size: 1rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  animation: sl-fade-in-up 0.6s ease 0.4s both;
}

/* Manifesto */
.manifesto {
  text-align: center;
  padding: var(--sl-space-lg);
}

.manifesto-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-md);
}

.manifesto-text {
  font-size: 0.9375rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  margin-bottom: var(--sl-space-sm);
}

/* Contributor Wall */
.contributor-section {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.section-header {
  text-align: center;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-xs);
}

.section-desc {
  font-size: 0.9375rem;
  color: var(--sl-text-tertiary);
}

.contributor-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--sl-space-md);
}

.contributor-card {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
  transition: all var(--sl-transition-normal);
}

.contributor-card.clickable {
  cursor: pointer;
}

.contributor-card.clickable:hover {
  transform: translateY(-4px);
  box-shadow: var(--sl-shadow-lg);
}

.contributor-avatar {
  width: 48px;
  height: 48px;
  border-radius: var(--sl-radius-md);
  flex-shrink: 0;
  background: var(--sl-bg-tertiary);
  image-rendering: pixelated;
}

.contributor-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.contributor-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.contributor-role {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

.join-card {
  border: 2px dashed var(--sl-border);
  background: transparent;
  cursor: default;
}

.join-card:hover {
  border-color: var(--sl-primary-light);
  background: var(--sl-primary-bg);
  transform: none;
  box-shadow: none;
}

.join-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.join-text {
  color: var(--sl-primary);
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-md);
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--sl-border-light);
}

.info-item:last-child { border-bottom: none; }

.info-label {
  font-size: 0.875rem;
  color: var(--sl-text-tertiary);
}

.info-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--sl-text-primary);
  font-family: var(--sl-font-mono);
}

/* Contribute Ways */
.contribute-ways {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-sm);
}

.way-item {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm);
  border-radius: var(--sl-radius-md);
  transition: background var(--sl-transition-fast);
}

.way-item:hover {
  background: var(--sl-bg-secondary);
}

.way-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
  width: 36px;
  text-align: center;
}

.way-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.way-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.way-desc {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

/* Links */
.links-section {
  display: flex;
  justify-content: center;
  gap: var(--sl-space-md);
}

/* Footer */
.about-footer {
  text-align: center;
  padding: var(--sl-space-xl) 0;
  border-top: 1px solid var(--sl-border-light);
}

.footer-text {
  font-size: 0.8125rem;
  color: var(--sl-text-tertiary);
  margin-bottom: var(--sl-space-xs);
}

.footer-quote {
  font-size: 1rem;
  font-weight: 500;
  color: var(--sl-primary);
  font-style: italic;
  margin-top: var(--sl-space-md);
}

@media (max-width: 768px) {
  .info-grid { grid-template-columns: 1fr; }
  .contribute-ways { grid-template-columns: 1fr; }
  .contributor-grid { grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); }
}
</style>
