# 桌面待办事项插件 - 设计文档

## 项目概述

一个可调节透明度的桌面待办事项小插件，采用卡片式设计，支持贴在系统桌面上。基于 Web 技术栈开发，可通过 Electron 打包为桌面应用。

---

## 设计理念

### 视觉风格
- **玻璃拟态 (Glassmorphism)**：半透明背景 + 模糊效果，与桌面环境自然融合
- **现代简约**：干净的线条，充足的留白，层次分明的信息结构
- **动态交互**：流畅的动画过渡，即时的用户反馈

### 核心特点
| 特点 | 说明 |
|------|------|
| 透明度调节 | 20%-100% 无极调节，支持预设快捷设置 |
| 置顶显示 | 始终保持在桌面最上层 |
| 卡片式布局 | 每个待办事项独立卡片，支持拖拽排序 |
| 数据持久化 | 本地存储，关闭后数据不丢失 |

---

## 功能规格

### 核心功能

#### 1. 待办事项管理
- [x] 添加待办事项
- [x] 标记完成/未完成
- [x] 删除待办事项
- [x] 拖拽排序
- [x] 双击编辑

#### 2. 透明度控制
- [x] 滑块无极调节 (20%-100%)
- [x] 预设快捷键 (30%, 50%, 85%, 100%)
- [x] 实时预览效果
- [x] 记忆上次设置

#### 3. 数据统计
- [x] 待办总数
- [x] 已完成数量
- [x] 待完成数量

### 扩展功能（后续版本）

- [ ] 优先级设置 (高/中/低)
- [ ] 到期提醒
- [ ] 分类/标签
- [ ] 主题切换
- [ ] 数据导出/导入
- [ ] 云同步
- [ ] 全局快捷键

---

## 技术架构

### 技术栈选择

| 层级 | 技术 | 说明 |
|------|------|------|
| 桌面框架 | Electron | 跨平台桌面应用，支持透明度、置顶等特性 |
| 前端框架 | Vue.js 3 | 响应式数据绑定，组件化开发 |
| 语言 | TypeScript | 类型安全，更好的开发体验 |
| 样式 | CSS3 + 自定义属性 | 现代CSS特性，玻璃拟态效果 |
| 存储 | localStorage / SQLite | 本地数据持久化 |

### 项目结构

```
todo-widget/
├── package.json
├── electron/
│   ├── main.js          # Electron 主进程
│   └── preload.js       # 预加载脚本
├── src/
│   ├── main.ts          # Vue 入口
│   ├── App.vue          # 根组件
│   ├── components/
│   │   ├── TodoInput.vue    # 输入组件
│   │   ├── TodoList.vue     # 列表组件
│   │   ├── TodoCard.vue     # 卡片组件
│   │   ├── OpacitySlider.vue# 透明度滑块
│   │   └── Stats.vue        # 统计组件
│   ├── composables/
│   │   └── useTodos.ts      # 待办逻辑
│   ├── store/
│   │   └── todoStore.ts     # 状态管理
│   └── styles/
│       └── global.css       # 全局样式
└── public/
    └── index.html
```

### Electron 主进程配置

```javascript
const createTodoWindow = () => {
  todoWindow = new BrowserWindow({
    width: 360,
    height: 520,
    frame: false,              // 无边框窗口
    transparent: true,         // 支持透明
    alwaysOnTop: true,         // 置顶显示
    skipTaskbar: true,         // 不显示在任务栏
    hasShadow: true,           // 窗口阴影
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js')
    }
  });
  
  todoWindow.loadFile('dist/index.html');
  todoWindow.setOpacity(0.85);
};
```

---

## UI/UX 设计

### 配色方案

```css
:root {
  --primary: #6C63FF;        /* 主色调 - 紫罗兰 */
  --primary-light: #8B85FF;
  --accent: #FF6B8A;         /* 强调色 - 珊瑚红 */
  --success: #4ECB71;        /* 成功色 - 绿色 */
  --warning: #FFB74D;        /* 警告色 - 橙色 */
  --danger: #FF5252;         /* 危险色 - 红色 */
  
  --bg-dark: #1A1A2E;        /* 背景深色 */
  --bg-medium: #16213E;      /* 背景中色 */
  --bg-light: #0F3460;       /* 背景浅色 */
  
  --glass-bg: rgba(255, 255, 255, 0.08);
  --glass-border: rgba(255, 255, 255, 0.15);
}
```

### 组件设计

#### 透明度滑块
- 范围：20% - 100%
- 步长：1%
- 预设值：30%, 50%, 85%, 100%
- 视觉：渐变色填充，圆形拖动钮

#### 待办卡片
```
┌──────────────────────────────────┐
│ [☐] 待办事项内容                [×]│
│     🕐 创建时间                  │
└──────────────────────────────────┘
```

#### 输入区域
```
┌─────────────────────┐ ┌──────────┐
│ 输入待办事项...      │ │ [+ 添加] │
└─────────────────────┘ └──────────┘
```

### 动画效果

| 动画 | 触发条件 | 效果 |
|------|----------|------|
| slideUp | 页面加载/卡片出现 | 从下方滑入 |
| fadeIn | 元素显示 | 透明度渐变 |
| scale | 按钮点击 | 缩放反馈 |
| slideOut | 删除待办 | 向右滑出消失 |

---

## 交互设计

### 操作方式

| 操作 | 方式 | 效果 |
|------|------|------|
| 添加待办 | 输入文字 + 回车/点击按钮 | 新卡片出现在列表顶部 |
| 完成待办 | 点击复选框 | 文字添加删除线，透明度降低 |
| 删除待办 | 点击删除按钮 | 卡片滑出动画后移除 |
| 编辑待办 | 双击文字 | 进入编辑模式 |
| 拖拽排序 | 长按拖动 | 调整卡片顺序 |
| 调节透明度 | 拖动滑块/点击预设 | 实时更新窗口透明度 |

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| Ctrl/Cmd + N | 聚焦输入框 |
| Enter | 添加待办 |
| Ctrl/Cmd + W | 关闭窗口（不退出） |
| Ctrl/Cmd + Q | 退出应用 |

---

## 数据模型

```typescript
interface Todo {
  id: number;           // 唯一标识（时间戳）
  text: string;         // 待办内容
  completed: boolean;   // 是否完成
  createdAt: string;    // 创建时间
  updatedAt?: string;   // 更新时间
  priority?: 'low' | 'medium' | 'high';  // 优先级
}

interface Settings {
  opacity: number;      // 透明度 0.2 - 1.0
  theme: string;        // 主题名称
  position: { x: number, y: number };  // 窗口位置
}
```

---

## 开发计划

### Phase 1: 基础框架 (1-2周)
- [ ] 初始化 Electron + Vue 项目
- [ ] 搭建主进程和渲染进程
- [ ] 实现基础窗口配置（透明、置顶、无边框）
- [ ] 待办数据模型定义

### Phase 2: 核心功能 (2-3周)
- [ ] 待办增删改查
- [ ] 透明度控制组件
- [ ] 数据持久化 (localStorage)
- [ ] 统计面板

### Phase 3: UI 优化 (1-2周)
- [ ] 玻璃拟态样式完善
- [ ] 动画效果实现
- [ ] 响应式适配
- [ ] 主题系统

### Phase 4: 打包发布 (1周)
- [ ] 图标和启动画面
- [ ] 自动更新配置
- [ ] 安装包打包 (Windows/Mac)
- [ ] 发布到 GitHub Releases

---

## 创意参考来源

- **Dribbble**: 搜索 "glassmorphism todo app" 获取视觉灵感
- **Pinterest**: 搜索 "desktop widget design" 参考布局
- **Mobbin**: 参考优秀待办应用的交互设计
- **Glassmorphism.app**: 玻璃拟态设计工具

---

## 技术难点与解决方案

### 1. 透明度与鼠标穿透
**问题**: 透明度过高时，内容难以辨认
**方案**: 设置最低透明度 20%，文字始终使用高对比度颜色

### 2. 窗口拖拽
**问题**: 无边框窗口无法移动
**方案**: 通过 CSS `-webkit-app-region: drag` 设置拖拽区域

### 3. 数据同步
**问题**: 多实例运行可能导致数据冲突
**方案**: 使用 single-instance-lock 限制单实例运行

---

## 文件清单

| 文件 | 说明 |
|------|------|
| design.html | 设计展示页面（包含交互原型） |
| index.html | 功能完整的待办应用（可直接运行） |
| design.md | 本设计文档 |
