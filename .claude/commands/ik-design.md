---
description: Create an HTML-based interactive UI design prototype through a manual workflow.
disable-model-invocation: true
---

# IK Design

UI 设计辅助, 生成基于 HTML 的可交互设计原型.

## 调用方式

```bash
/ik-design
```

## 工作流程

### 1. 设计范围确认

与开发者确认设计范围:

- **设计模块**: 需要设计的页面/模块列表
- **设计风格**: 设计风格参考 (简洁 / 商务 / 创意 / 暗色等)
- **响应式**: 移动端适配需求
- **组件库**: 使用的 UI 组件库 (Tailwind CSS / 其他)

### 2. 设计目录结构

创建 `design/` 目录, 按以下结构组织:

```text
design/
├── shared/          # 共享组件 (布局、导航、通用组件)
├── [module-1]/      # 功能模块 1
│   ├── page-a/      # 页面 A
│   │   ├── index.html
│   │   └── components/
│   └── page-b/
├── [module-2]/      # 功能模块 2
└── start.sh         # 预览服务器启动脚本
```

### 3. 设计文件规范

- **入口文件**: 每个页面使用 `index.html` 作为入口
- **组件拆分**: 可复用组件拆分为独立 HTML 文件, 通过 fetch 动态加载
- **样式方案**: 推荐使用 Tailwind CSS CDN, 样式内联便于预览
- **图标方案**: 使用 Lucide Icons 或其他轻量图标库
- **交互脚本**: 简单交互直接内联, 复杂逻辑拆分为独立 js 文件

### 4. 预览服务器

生成 `start.sh` 启动 HTTP 预览服务器:

```bash
#!/bin/bash
cd "$(dirname "$0")"
echo "Design preview server starting at http://localhost:8090"
python3 -m http.server 8090
```

### 5. 输出要求

- 每个页面提供完整 HTML, 可直接在浏览器预览
- 使用语义化 HTML 标签
- 注释清晰, 标明组件边界和用途
- 关键交互状态 (hover / active / focus) 需体现
- 移动端适配需考虑断点 (`sm` / `md` / `lg` / `xl`)
