import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "home",
    component: () => import("../views/HomeView.vue"),
    meta: { title: "首页", icon: "home" },
  },
  {
    path: "/components",
    name: "components",
    component: () => import("../views/ComponentsView.vue"),
    meta: { title: "组件管理", icon: "package" },
  },
  {
    path: "/inject",
    name: "inject",
    component: () => import("../views/InjectView.vue"),
    meta: { title: "注入", icon: "zap" },
  },
  {
    path: "/settings",
    name: "settings",
    component: () => import("../views/SettingsView.vue"),
    meta: { title: "设置", icon: "sliders" },
  },
  {
    path: "/about",
    name: "about",
    component: () => import("../views/AboutView.vue"),
    meta: { title: "关于", icon: "info" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
