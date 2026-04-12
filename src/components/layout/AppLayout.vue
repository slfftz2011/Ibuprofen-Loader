<template>
  <div class="app-layout">
    <!-- Sidebar -->
    <AppSidebar />

    <!-- Main Content -->
    <div class="main-content">
      <!-- Header -->
      <AppHeader />

      <!-- Router View -->
      <main class="router-view">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppSidebar from './AppSidebar.vue';
import AppHeader from './AppHeader.vue';
</script>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--sl-bg);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: var(--sl-sidebar-width);
  transition: margin-left var(--sl-transition-normal);
  min-width: 0;
}

.router-view {
  flex: 1;
  padding: var(--sl-space-lg);
  overflow-y: auto;
  overflow-x: hidden;
}
</style>

