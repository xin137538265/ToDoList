# TodoList Widget - Tauri 2.0 桌面应用 Spec

## Why
将现有的 HTML/CSS/JS 待办事项 Widget 改造为基于 Tauri 2.0 的轻量级桌面应用，实现常驻后台、开机启动、系统托盘、透明窗口等桌面特性，打包体积小（3-8MB）、内存占用低（10-30MB），可打包后分发给同事使用。

## What Changes
- 在现有项目根目录初始化 Tauri 2.0 项目
- 迁移 index-v2.html 到 Tauri 前端
- 配置 Tauri 窗口属性（透明、置顶、无边框、可拖拽）
- 实现系统托盘（图标、右键菜单、点击恢复、关闭隐藏）
- 实现窗口拖拽区域和位置记忆
- 添加开机自启功能（设置开关）
- 添加全局快捷键（唤出/隐藏窗口）
- 配置应用图标和打包产物

## Impact
- 新增: `src-tauri/` 整个目录
- 新增: `package.json`, `src/index.html`
- 修改: 前端代码需适配 Tauri WebView 环境（Google Fonts 本地化、CSS 拖拽区域）
- 新增依赖: @tauri-apps/cli, tauri-plugin-autostart, tauri-plugin-global-shortcut, tauri-plugin-single-instance

## ADDED Requirements

### Requirement: Tauri 2.0 项目结构
项目 SHALL 包含完整的 Tauri 2.0 后端结构：
- `src-tauri/tauri.conf.json` — 窗口配置、打包配置
- `src-tauri/Cargo.toml` — Rust 依赖
- `src-tauri/capabilities/default.json` — 权限声明
- `src-tauri/src/main.rs` — 主入口逻辑
- `src-tauri/icons/` — 应用图标

#### Scenario: 项目初始化
- **WHEN** 运行 `npm run tauri dev`
- **THEN** 应用启动，显示透明无边框窗口

### Requirement: 窗口配置
窗口 SHALL 配置以下属性：
- 宽度 420px, 高度 640px
- 无边框 (`decorations: false`)
- 透明 (`transparent: true`)
- 置顶 (`alwaysOnTop: true`)
- 隐藏任务栏图标 (`skipTaskbar: true`)
- 可拖拽 (`draggable: true` 或通过 CSS `-webkit-app-region: drag`)

#### Scenario: 窗口启动
- **WHEN** 应用启动
- **THEN** 窗口出现在桌面右上角，置顶显示

### Requirement: 系统托盘
应用 SHALL 实现系统托盘功能：
- 托盘图标显示应用 logo
- 右键菜单：显示窗口 / 退出
- 点击托盘图标：显示窗口
- 关闭窗口时隐藏到托盘（不退出进程）

#### Scenario: 关闭窗口
- **WHEN** 用户点击窗口关闭按钮
- **THEN** 窗口隐藏到系统托盘，进程继续运行

#### Scenario: 退出应用
- **WHEN** 用户点击托盘右键菜单的"退出"选项
- **THEN** 应用完全退出

### Requirement: 开机自启
应用 SHALL 支持开机自启：
- 设置面板中可开关开机自启
- 默认关闭
- 使用 `tauri-plugin-autostart` 实现

#### Scenario: 开启自启
- **WHEN** 用户在设置中开启"开机自启"
- **THEN** 下次开机自动启动

### Requirement: 窗口拖拽
应用 SHALL 支持拖拽移动窗口：
- header 区域设置为可拖拽区域
- 按钮、输入框等交互元素排除拖拽区域

#### Scenario: 拖拽移动
- **WHEN** 用户在 header 区域按下鼠标拖动
- **THEN** 窗口跟随鼠标移动

### Requirement: 全局快捷键
应用 SHALL 支持全局快捷键：
- `CmdOrCtrl+Shift+T` — 唤出/隐藏窗口

#### Scenario: 全局唤出窗口
- **WHEN** 应用已最小化到托盘
- **AND** 用户按下 Cmd+Shift+T
- **THEN** 窗口显示

### Requirement: 前端适配
前端代码 SHALL 适配 Tauri WebView 环境：
- Google Fonts 改为本地引用或使用系统字体回退
- header 区域添加 `-webkit-app-region: drag` 拖拽样式
- 交互元素添加 `-webkit-app-region: no-drag` 排除拖拽
- 移除浏览器专用 meta 标签（如 viewport）

#### Scenario: 字体回退
- **WHEN** Google Fonts 无法加载（离线环境）
- **THEN** 使用系统字体回退（PingFang SC, Microsoft YaHei）

## MODIFIED Requirements

### Requirement: 数据持久化
localStorage SHALL 继续使用，Tauri WebView 原生支持。后续版本可扩展为 Tauri 文件存储。

### Requirement: 透明度调节
当前通过 `document.querySelectorAll('.widget-card').forEach(...)` 调节元素透明度，后续可扩展为调节整个 Tauri 窗口透明度（`window.set_shadow()` + 背景色控制）。

## REMOVED Requirements
无。所有现有前端功能完整保留。
