 Ibuprofen Loader v3
 
<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.94.0-orange.svg)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3-green.svg)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)

[![Github last commit](https://img.shields.io/github/last-commit/slfftz2011/Ibuprofen-Loader)](https://github.com/slfftz2011/Ibuprofen-Loader/commits/)
[![Github commit activity](https://img.shields.io/github/commit-activity/w/slfftz2011/Ibuprofen-Loader)](https://github.com/slfftz2011/Ibuprofen-Loader/activity/)
[![Github contributors](https://img.shields.io/github/contributors/slfftz2011/Ibuprofen-Loader)](https://github.com/slfftz2011/Ibuprofen-Loader/contributors/)

![Github code size](https://img.shields.io/github/languages/code-size/slfftz2011/Ibuprofen-Loader)
![GitHub repo size](https://img.shields.io/github/repo-size/slfftz2011/Ibuprofen-Loader)
![Github lines of code](https://5ezz6jithh.execute-api.us-east-1.amazonaws.com/prod/lambda-shield-redirect?user=slfftz2011&repo=Ibuprofen-Loader)

[![GitHub Downloads](https://img.shields.io/github/downloads/slfftz2011/Ibuprofen-Loader/total)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/)
[![GitHub Tag](https://img.shields.io/github/v/tag/slfftz2011/Ibuprofen-Loader)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/)
![GitHub Repo stars](https://img.shields.io/github/stars/slfftz2011/Ibuprofen-Loader)

</div>

## 现代化网易我的世界组件注入器

**Ibuprofen Loader** 是 C++ 版本 [Netease Mod Injector 2](https://github.com/slfftz2011/NeteaseModInjector2) 的重写，使用 **Rust + Vue 3 + TypeScript** 技术栈，支持更多功能。


### 📦 快速开始

1. **下载最新版本**：

 **最新测试版** : [![最新测试版](https://img.shields.io/github/v/tag/slfftz2011/Ibuprofen-Loader?include_prereleases)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/tag/3.0.0-snapshot-2)

 **最新正式版** : 敬请期待<!--[![最新正式版](https://img.shields.io/github/v/tag/slfftz2011/Ibuprofen-Loader)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/tag/3.0.0-snapshot-2) -->

 **全部版本** :

[![v3.0.0-snapshot-2](https://img.shields.io/badge/download-v3.0.0--snapshot--2-orange)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/tag/3.0.0-snapshot-2)
[![v3.0.0-snapshot-1](https://img.shields.io/badge/download-v3.0.0--snapshot--1-orange)](https://github.com/slfftz2011/Ibuprofen-Loader/releases/tag/3.0.0-snapshot-1)


2. **准备组件**：
```
Ibuprofen Loader/components/*.COP
```

3. **运行**：
```
ibuprofen_loader.exe
```

### 🛠️ 开发环境

```bash
# 克隆项目
git clone "https://github.com/slfftz2011/Ibuprofen Loader"

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```


### 📁 项目结构

```
Ibuprofen Loader/
├── src/                    # Vue 前端
│   ├── views/             # 页面组件
│   ├── stores/            # Pinia 状态
│   └── api/               # Tauri API
├── src-tauri/             # Rust 后端
│   ├── core/              # 业务逻辑
│   └── commands/          # Tauri 命令
└── README.md              # 本文档
```

### 🔧 核心功能

1. **自动检测网易MC路径** (注册表读取)
2. **网络状态检查** + 加速器推荐
3. **COP组件扫描** + 完整性验证
4. **一键注入** (mods/config/resourcepacks)
5. **自动备份** + 日志触发机制


### 🛠️ 构建配置

- **Rust**: 1.80+ (stable)
- **Node**: 20+ 
- **Tauri**: v2.10+
- **Vue**: 3.5+

### 📄 许可证

[MIT License](LICENSE)


### 🤝 鸣谢

- **闪烁的红石君** - 原始技术支持
- **Tauri 团队** - 跨平台框架
- **Rust/Vue 社区** - 优秀生态

### 📞 联系

```
作者: SLFFTZ520
邮箱: slfftz520@163.com
GitHub: https://github.com/slfftz2011
```

---
*Ibuprofen Loader - 布洛芬，给你的网易降降温！*

