# TodoList Widget

轻量级桌面待办事项 Widget，玻璃拟态设计，常驻桌面，开机可启。

## 特性

- **轻量级** — 基于 Tauri 2.0，打包 3-8MB，内存 10-30MB
- **玻璃拟态** — 半透明背景 + 模糊效果，与桌面自然融合
- **透明度调节** — 20%-100% 无极调节，支持预设快捷设置
- **置顶显示** — 始终保持在桌面最上层
- **常驻后台** — 关闭窗口后驻留系统托盘，不退出进程
- **开机启动** — 可选开机自动启动
- **截止时间** — 快捷设置 + 自定义选择，动态倒计时显示
- **优先级** — 高/中/低三级，标题行内联显示
- **任务筛选** — 待完成/已完成/全部，统计卡片联动切换
- **拖拽排序** — 拖拽调整待办顺序
- **数据持久化** — 本地存储，关闭后数据不丢失

## 截图

![TodoList Widget](docs/screenshot.png)

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.0 |
| 前端 | 原生 HTML / CSS / JS |
| 后端 | Rust |
| 存储 | localStorage |

## 开发

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- [Tauri CLI](https://tauri.app/start/prerequisites/) 2.0

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Rust 依赖（Tauri 自动管理）
cd src-tauri && cargo build
```

### 开发模式

```bash
npm run tauri dev
```

### 构建打包

```bash
npm run tauri build
```

产物位于 `src-tauri/target/release/bundle/`：
- macOS: `.dmg`
- Windows: `.msi` / `.exe`

## 项目结构

```
todo-widget/
├── src-tauri/              # Tauri 后端（Rust）
│   ├── src/main.rs         # 窗口配置、托盘、自启、快捷键
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
├── src/                    # 前端
│   └── index.html          # 主页面
├── package.json
├── design.md               # 设计文档
└── README.md
```

## 使用说明

### 基本操作

| 操作 | 方式 |
|------|------|
| 添加待办 | 输入文字 + 回车 / 点击添加按钮 |
| 完成待办 | 点击复选框 |
| 删除待办 | 悬浮卡片 → 点击删除按钮 |
| 编辑待办 | 双击卡片 / 点击编辑按钮 |
| 拖拽排序 | 长按拖动卡片 |
| 设置截止时间 | 编辑弹窗 → 快捷按钮或自定义选择 + 确认 |
| 切换筛选 | 点击统计卡片（待完成/已完成/总计） |
| 调节透明度 | 设置面板 → 拖动滑块 / 点击预设 |

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| Ctrl/Cmd + N | 聚焦输入框 |
| Ctrl/Cmd + W | 隐藏窗口到托盘 |
| Ctrl/Cmd + Q | 退出应用 |
| Ctrl/Cmd + Shift + T | 全局唤出/隐藏窗口 |

### 系统托盘

- 关闭窗口时自动最小化到系统托盘
- 点击托盘图标恢复窗口
- 右键托盘图标：显示窗口 / 退出

## 许可证

MIT
