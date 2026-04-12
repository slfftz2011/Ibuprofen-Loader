<script setup lang="ts">
import { ref, onMounted } from "vue";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import SLInput from "../components/common/SLInput.vue";
import SLSelect from "../components/common/SLSelect.vue";
import SLSwitch from "../components/common/SLSwitch.vue";
import SLModal from "../components/common/SLModal.vue";
import type { AppSettings } from "../types";
import * as settingsApi from "../api/settings";

const settings = ref<AppSettings>({
  componentsDir: "",
  autoBackup: true,
  manualGamePath: "",
  theme: "auto",
});
const loading = ref(false);
const saving = ref(false);
const error = ref<string | null>(null);
const success = ref<string | null>(null);
const hasChanges = ref(false);
const showResetConfirm = ref(false);
const showImportModal = ref(false);
const importJson = ref("");

const themeOptions = [
  { label: "自动", value: "auto" },
  { label: "亮色", value: "light" },
  { label: "暗色", value: "dark" },
];

async function loadSettings() {
  loading.value = true;
  error.value = null;
  try {
    settings.value = await settingsApi.get();
    hasChanges.value = false;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function markChanged() {
  hasChanges.value = true;
}

async function saveSettings() {
  saving.value = true;
  error.value = null;
  try {
    await settingsApi.save(settings.value);
    success.value = "设置已保存";
    hasChanges.value = false;
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function resetSettings() {
  try {
    settings.value = await settingsApi.reset();
    showResetConfirm.value = false;
    hasChanges.value = false;
    success.value = "已恢复默认设置";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

async function exportSettings() {
  try {
    const json = await settingsApi.exportJson();
    await navigator.clipboard.writeText(json);
    success.value = "设置 JSON 已复制到剪贴板";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

async function handleImport() {
  if (!importJson.value.trim()) {
    error.value = "请粘贴 JSON";
    return;
  }
  try {
    settings.value = await settingsApi.importJson(importJson.value);
    showImportModal.value = false;
    importJson.value = "";
    hasChanges.value = false;
    success.value = "设置已导入";
    setTimeout(() => (success.value = null), 3000);
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(loadSettings);
</script>

<template>
  <div class="settings-view animate-fade-in-up">
    <div v-if="error" class="msg-banner error-banner">
      <span>{{ error }}</span>
      <button @click="error = null">x</button>
    </div>
    <div v-if="success" class="msg-banner success-banner">
      <span>{{ success }}</span>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <span>加载设置...</span>
    </div>

    <template v-else>
      <SLCard title="应用设置" subtitle="保存 Ibuprofen Loader 的本地配置">
        <div class="settings-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">组件目录</span>
              <span class="setting-desc">放置 .COP 文件的目录，用于自动扫描与注入。</span>
            </div>
            <div class="input-lg">
              <SLInput
                v-model="settings.componentsDir"
                placeholder="例如：C:\\Ibuprofen Loader\\components"
                @update:modelValue="markChanged"
              />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">手动游戏路径</span>
              <span class="setting-desc">如果自动检测失败，可手动指定网易我的世界安装目录。</span>
            </div>
            <div class="input-lg">
              <SLInput
                v-model="settings.manualGamePath"
                placeholder="例如：C:\\Program Files\\Netease\\Minecraft Launcher"
                @update:modelValue="markChanged"
              />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">自动备份</span>
              <span class="setting-desc">注入前自动备份 mods/config/resourcepacks，便于恢复。</span>
            </div>
            <SLSwitch v-model="settings.autoBackup" @update:modelValue="markChanged" />
          </div>

          <div class="setting-row full-width">
            <div class="setting-info">
              <span class="setting-label">界面主题</span>
              <span class="setting-desc">选择应用主题，或使用系统自动模式。</span>
            </div>
            <div class="input-lg">
              <SLSelect
                v-model="settings.theme"
                :options="themeOptions"
                @update:modelValue="markChanged"
              />
            </div>
          </div>
        </div>
      </SLCard>

      <div class="settings-actions">
        <div class="actions-left">
          <SLButton variant="primary" size="lg" :loading="saving" @click="saveSettings">
            保存设置
          </SLButton>
          <SLButton variant="secondary" @click="loadSettings">放弃修改</SLButton>
          <span v-if="hasChanges" class="unsaved-hint">有未保存的更改</span>
        </div>
        <div class="actions-right">
          <SLButton variant="ghost" size="sm" @click="exportSettings">导出</SLButton>
          <SLButton variant="ghost" size="sm" @click="showImportModal = true">导入</SLButton>
          <SLButton variant="danger" size="sm" @click="showResetConfirm = true">恢复默认</SLButton>
        </div>
      </div>
    </template>

    <SLModal :visible="showImportModal" title="导入设置" @close="showImportModal = false">
      <div class="import-form">
        <p class="text-caption">粘贴之前导出的 JSON 数据</p>
        <textarea
          class="import-textarea"
          v-model="importJson"
          placeholder='{"componentsDir":"...","autoBackup":true,"manualGamePath":"...","theme":"auto"}'
          rows="10"
        ></textarea>
      </div>
      <template #footer>
        <SLButton variant="secondary" @click="showImportModal = false">取消</SLButton>
        <SLButton variant="primary" @click="handleImport">导入</SLButton>
      </template>
    </SLModal>

    <SLModal :visible="showResetConfirm" title="确认恢复默认" @close="showResetConfirm = false">
      <p class="text-body">确定要将所有设置恢复为默认值吗？此操作不可撤销。</p>
      <template #footer>
        <SLButton variant="secondary" @click="showResetConfirm = false">取消</SLButton>
        <SLButton variant="danger" @click="resetSettings">确认恢复</SLButton>
      </template>
    </SLModal>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex; flex-direction: column; gap: var(--sl-space-lg);
  max-width: 860px; padding-bottom: var(--sl-space-2xl);
}

.msg-banner {
  display: flex; align-items: center; justify-content: space-between;
  padding: 10px 16px; border-radius: var(--sl-radius-md); font-size: 0.875rem;
}
.error-banner { background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.2); color: var(--sl-error); }
.success-banner { background: rgba(34,197,94,0.1); border: 1px solid rgba(34,197,94,0.2); color: var(--sl-success); }
.msg-banner button { font-weight: 600; color: inherit; }

.loading-state {
  display: flex; align-items: center; justify-content: center;
  gap: var(--sl-space-sm); padding: var(--sl-space-2xl); color: var(--sl-text-tertiary);
}
.spinner { width: 18px; height: 18px; border: 2px solid var(--sl-border); border-top-color: var(--sl-primary); border-radius: 50%; animation: sl-spin 0.8s linear infinite; }

.settings-group { display: flex; flex-direction: column; }

.setting-row {
  display: flex; align-items: center; justify-content: space-between;
  padding: var(--sl-space-md) 0; border-bottom: 1px solid var(--sl-border-light);
  gap: var(--sl-space-lg);
}
.setting-row:last-child { border-bottom: none; }
.setting-row.full-width { flex-direction: column; align-items: stretch; }

.setting-info { flex: 1; display: flex; flex-direction: column; gap: 2px; min-width: 0; }
.setting-label { font-size: 0.9375rem; font-weight: 500; color: var(--sl-text-primary); }
.setting-desc { font-size: 0.8125rem; color: var(--sl-text-tertiary); line-height: 1.4; }

.input-sm { width: 120px; flex-shrink: 0; }
.input-lg { width: 100%; max-width: 420px; flex-shrink: 0; }

.import-textarea {
  width: 100%; margin-top: var(--sl-space-sm);
  padding: var(--sl-space-sm) var(--sl-space-md);
  font-family: var(--sl-font-mono); font-size: 0.8125rem;
  color: var(--sl-text-primary); background: var(--sl-surface);
  border: 1px solid var(--sl-border); border-radius: var(--sl-radius-md);
  resize: vertical; line-height: 1.6;
}
.import-textarea:focus {
  border-color: var(--sl-primary); box-shadow: 0 0 0 3px var(--sl-primary-bg); outline: none;
}

.settings-actions {
  display: flex; align-items: center; justify-content: space-between;
  padding: var(--sl-space-md) 0; border-top: 1px solid var(--sl-border);
}
.actions-left, .actions-right { display: flex; align-items: center; gap: var(--sl-space-sm); }

.unsaved-hint {
  font-size: 0.8125rem; color: var(--sl-warning); font-weight: 500;
  padding: 2px 10px; background: rgba(245,158,11,0.1); border-radius: var(--sl-radius-full);
}

.import-form { display: flex; flex-direction: column; gap: var(--sl-space-md); }
</style>
