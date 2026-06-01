# Tasks

## Phase 1: Tauri 项目初始化 + 前端迁移

- [x] Task 1: 初始化 Tauri 2.0 项目结构
  - [x] Step 1.1: 创建 `package.json`，安装 `@tauri-apps/cli` 2.x
  - [x] Step 1.2: 运行 `npx tauri init` 初始化 `src-tauri/` 目录（或手动创建）
  - [x] Step 1.3: 配置 `src-tauri/tauri.conf.json`（窗口、打包、构建路径）
  - [x] Step 1.4: 配置 `src-tauri/Cargo.toml`（Rust 依赖）
  - [x] Step 1.5: 创建 `src-tauri/capabilities/default.json`（权限声明）
  - [x] Step 1.6: 创建 `src-tauri/src/main.rs`（最小程序入口）
  - [x] Step 1.7: 复制 `index-v2.html` 到 `src/index.html`
  - [x] Step 1.8: 运行 `npm run tauri dev` 验证窗口能正常启动

- [x] Task 2: 前端代码适配 Tauri WebView 环境
  - [x] Step 2.1: 移除 `<meta name="viewport">`（WebView 不需要）
  - [x] Step 2.2: 将 Google Fonts 改为 `<link>` 异步加载 + 系统字体回退
  - [x] Step 2.3: header 区域添加 `-webkit-app-region: drag`
  - [x] Step 2.4: 所有交互元素（按钮、输入框、滑块等）添加 `-webkit-app-region: no-drag`
  - [x] Step 2.5: 验证所有前端功能在 Tauri WebView 中正常运行

## Phase 2: 桌面集成

- [x] Task 3: 实现系统托盘
  - [x] Step 3.1: 在 `tauri.conf.json` 配置 `trayIcon`
  - [x] Step 3.2: 在 `main.rs` 添加 `tray` 插件和菜单
  - [x] Step 3.3: 实现关闭事件拦截（隐藏窗口而非退出）
  - [x] Step 3.4: 实现托盘菜单事件（显示窗口/退出）
  - [x] Step 3.5: 创建托盘图标文件（PNG 格式，32x32）

- [x] Task 4: 实现窗口位置记忆
  - [x] Step 4.1: 在 `main.rs` 监听窗口移动事件
  - [x] Step 4.2: 保存窗口位置到 ~/.todo-widget/window.json
  - [x] Step 4.3: 启动时恢复窗口位置

- [x] Task 5: 实现开机自启
  - [x] Step 5.1: 添加 `tauri-plugin-autostart` 依赖
  - [x] Step 5.2: 在 `main.rs` 注册 autostart 插件
  - [x] Step 5.3: 在设置面板添加"开机自启"开关 UI
  - [x] Step 5.4: 通过 `invoke` 调用 Rust 端开关函数
  - [x] Step 5.5: 验证自启功能（macOS 和 Windows）

- [x] Task 6: 实现全局快捷键
  - [x] Step 6.1: 添加 `tauri-plugin-global-shortcut` 依赖
  - [x] Step 6.2: 在 `main.rs` 注册快捷键 `CmdOrCtrl+Shift+T`
  - [x] Step 6.3: 实现快捷键回调（显示/隐藏窗口）
  - [x] Step 6.4: 验证快捷键功能

## Phase 3: 打包发布

- [x] Task 7: 应用图标 + 打包
  - [x] Step 7.1: 生成应用图标（各尺寸 PNG + ICNS/ICO）
  - [x] Step 7.2: 配置 `tauri.conf.json` 中的 `bundle` 设置
  - [x] Step 7.3: 运行 `npm run tauri build` 生成安装包
  - [x] Step 7.4: 验证 `.dmg` 安装包能否正常安装和运行
