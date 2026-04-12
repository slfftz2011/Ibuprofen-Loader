// Pinia store for app state management
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type {
  ConnectionStatus,
  CopFileInfo,
  ListJsonInfo,
  WebsiteCheckResult,
} from "../types";
import * as api from "../api/tauri";

export const useAppStore = defineStore("app", () => {
  // State
  const gamePath = ref<string | null>(null);
  const networkStatus = ref<ConnectionStatus>({status: 'checking', message: '检测中...'});
  const githubStatus = ref<WebsiteCheckResult>({reachable: false, error: '检测中...'});
  const components = ref<CopFileInfo[]>([]);
  const selectedComponent = ref<CopFileInfo | null>(null);
  const extractedComponent = ref<string | null>(null);
  const componentInfo = ref<ListJsonInfo | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
// toast handled by composable


  const currentView = ref<"home" | "components" | "inject" | "settings">("home");
  const isInjecting = ref(false);
  const injectionProgress = ref(0);
  const injectionMessage = ref("");

  // Computed
  const hasGamePath = computed(() => gamePath.value !== null);
  const isNetworkConnected = computed(
    () => networkStatus.value?.status === "connected"
  );
  const hasComponents = computed(() => components.value.length > 0);
  const githubConnected = computed(() => githubStatus.value?.reachable ?? false);

  // Actions
async function initialize() {
    // Instant startup with immediate checks
    currentView.value = 'home';
    
    // Immediately start network check and other inits
    checkNetworkBackground();
    // Note: gamePath and components checked in App.vue onMounted
  }

  // Background network check with error fallback
  async function checkNetworkBackground() {
    try {
      const status = await api.checkConnectionStatus();
      networkStatus.value = status;
      githubStatus.value = await api.checkWebsiteReachable("https://github.com");
    } catch (e) {
      console.error('Network check failed:', e);
      // Fallback to disconnected on error
      networkStatus.value = { status: 'disconnected' as const, message: '检测失败' };
      githubStatus.value = { reachable: false, error: '网络检测失败' };
    }
  }

  async function testGithubConnectivity() {
    try {
      const result = await api.checkWebsiteReachable("https://github.com");

      githubStatus.value = result;
    } catch (e) {
      githubStatus.value = { reachable: false, error: "测试失败" };
    }
  }


  async function openExternalUrl(url: string) {
    try {
      await api.openUrl(url);
    } catch (e) {
      error.value = e instanceof Error ? e.message : "无法打开链接";
    }
  }
  
  async function scanComponents() {
    try {
      components.value = await api.scanCopFiles();
    } catch (e) {
      error.value = e instanceof Error ? e.message : "扫描组件失败";
    }
  }

  async function selectComponent(component: CopFileInfo) {
    selectedComponent.value = component;
    isLoading.value = true;
    error.value = null;

    try {
      const result = await api.processCopFile(component.path);
      if (result.success && result.outputPath) {
        extractedComponent.value = result.outputPath!;
        componentInfo.value = await api.loadListJson(result.outputPath!);
      } else {
        error.value = result.error || "处理组件失败";
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : "选择组件失败";
    } finally {
      isLoading.value = false;
    }
  }

  async function startInjection() {
    if (!gamePath.value || !extractedComponent.value) {
      error.value = "游戏路径或组件路径无效";
      return;
    }

    isInjecting.value = true;
    injectionProgress.value = 0;
    injectionMessage.value = "正在备份现有文件...";
    error.value = null;

    try {
      injectionProgress.value = 20;
      await api.backupDirectories(gamePath.value);

      injectionMessage.value = "等待游戏启动...";
      injectionProgress.value = 40;

      const launched = await api.waitForGameLaunch(60);
      if (!launched) {
        throw new Error("等待游戏启动超时");
      }

      injectionMessage.value = "正在注入组件...";
      injectionProgress.value = 60;

      await api.copyFilesToGame(
        `${extractedComponent.value}/mods`,
        `${extractedComponent.value}/config`,
        `${extractedComponent.value}/resourcepacks`,
        gamePath.value
      );

      injectionProgress.value = 100;
      injectionMessage.value = "注入完成！";
    } catch (e) {
      error.value = e instanceof Error ? e.message : "注入失败";
    } finally {
      isInjecting.value = false;
    }
  }

// toast handled by composable


  function setView(view: "home" | "components" | "inject" | "settings") {
    currentView.value = view;
  }

  function clearError() {
    error.value = null;
  }

  function clearSelection() {
    selectedComponent.value = null;
    extractedComponent.value = null;
    componentInfo.value = null;
  }

  // Periodic network polling interval - use number for TS compatibility
  const networkPollingInterval = ref<number | null>(null);

  // Start periodic network checks
  function startNetworkPolling() {
    if (networkPollingInterval.value) return; // already running
    
    // Initial check immediate
    checkNetworkBackground();
    
    // Poll every 10 seconds
    networkPollingInterval.value = setInterval(() => {
      checkNetworkBackground();
    }, 10000);
  }

  // Stop polling (cleanup)
  function stopNetworkPolling() {
    if (networkPollingInterval.value) {
      clearInterval(networkPollingInterval.value);
      networkPollingInterval.value = null;
    }
  }

return {
    gamePath,
    networkStatus,
    githubStatus,
    components,
    selectedComponent,
    extractedComponent,
    componentInfo,
    isLoading,
    error,
    // toast handled externally

    currentView,
    isInjecting,
    injectionProgress,
    injectionMessage,
    hasGamePath,
    isNetworkConnected,
    hasComponents,
    githubConnected,
    initialize,
    checkNetworkBackground,
    testGithubConnectivity,
    scanComponents,
    selectComponent,
    startInjection,
    setView,
    clearError,
    clearSelection,
    openExternalUrl,
    
    // New polling methods
    startNetworkPolling,
    stopNetworkPolling,
  };


});

