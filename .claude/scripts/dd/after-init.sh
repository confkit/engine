#!/bin/bash

# DD项目初始化后处理脚本
# 职责: 文件操作、状态管理、环境检查
# 由 dd:init 命令调用，接收 AI 生成的结构化项目数据

set -e

echo "🔧 DD项目初始化后处理..."
echo ""

# 解析并验证 JSON 参数
init_data="$1"
if [ -z "$init_data" ] || [ "$init_data" = "null" ]; then
    echo "❌ 错误: 缺少项目初始化数据"
    echo "用法: bash after-init.sh '<structured_project_data>'"
    exit 1
fi

# 验证 JSON 格式
if ! echo "$init_data" | jq empty 2>/dev/null; then
    echo "❌ 错误: JSON数据格式无效"
    exit 1
fi

# 提取核心项目信息
project_name=$(echo "$init_data" | jq -r '.project_name // "未命名项目"')
project_type=$(echo "$init_data" | jq -r '.project_type // "应用项目"')
tech_stack=$(echo "$init_data" | jq -r '.tech_stack // "待确定"')
architecture=$(echo "$init_data" | jq -r '.architecture // "待设计"')
conversation=$(echo "$init_data" | jq -r '.conversation // ""')

# 提取 AI 生成的具体内容
project_content=$(echo "$init_data" | jq -r '.project_content // ""')
tech_content=$(echo "$init_data" | jq -r '.tech_content // ""') 
architecture_content=$(echo "$init_data" | jq -r '.architecture_content // ""')
requirements_content=$(echo "$init_data" | jq -r '.requirements_content // ""')
status_content=$(echo "$init_data" | jq -r '.status_content // ""')

# 参数验证和提示
validate_content() {
    local content="$1"
    local type="$2"
    
    if [ -z "$content" ] || [ "$content" = "" ]; then
        echo "⚠️  警告: 缺少${type}内容，将使用默认模板"
        return 1
    fi
    return 0
}

echo "📋 项目配置信息:"
echo "  项目名称: $project_name"
echo "  项目类型: $project_type" 
echo "  技术栈: $tech_stack"
echo "  架构模式: $architecture"
echo ""

# 验证 AI 生成的内容完整性
echo "🔍 验证AI生成内容完整性..."
validate_content "$project_content" "项目描述"
validate_content "$tech_content" "技术栈详情" 
validate_content "$architecture_content" "架构设计"
validate_content "$requirements_content" "需求文档"
validate_content "$status_content" "项目状态"
echo ""

## 1. 记录初始化对话历史

if [ -n "$conversation" ] && [ "$conversation" != "" ]; then
    echo "💬 保存初始化对话记录..."
    
    # 确保对话目录存在
    mkdir -p .claude/chats/init
    
    # 生成对话文件
    chat_filename="init-$(date +"%Y%m%d-%H%M%S").md"
    chat_filepath=".claude/chats/init/$chat_filename"
    
    # 写入对话记录
    cat > "$chat_filepath" << EOF
---
type: communicate
project_name: $project_name
participants: [user, ai]
created_at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
---

# 项目初始化对话记录

## 项目信息

- **项目名称**: $project_name
- **项目类型**: $project_type
- **技术栈**: $tech_stack
- **架构模式**: $architecture

## 初始化对话

$conversation

## 对话成果

通过深度对话确定了项目的核心信息和技术决策，建立了项目发展的基础框架。

## 后续行动

系统已基于对话结果生成项目上下文文件，DD工作流系统配置完成。
EOF
    
    echo "  ✅ 对话记录保存至: $chat_filepath"
else
    echo "ℹ️  无对话历史，跳过对话记录步骤"
fi
echo ""

## 2. 生成项目上下文文件

echo "📝 生成项目上下文文件..."

# 生成 project.md - 使用AI内容或回退到模板
if validate_content "$project_content" >/dev/null 2>&1; then
    # 使用 AI 生成的内容
    cat > .claude/context/project.md << EOF
---
name: $project_name
version: 1.0.0
type: $project_type
status: 开发中
initialized_at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
---

$project_content
EOF
    echo "  ✅ project.md (AI生成内容)"
else
    # 使用基础模板
    cat > .claude/context/project.md << EOF
---
name: $project_name
version: 1.0.0
type: $project_type
status: 开发中
initialized_at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
---

# $project_name

## 项目描述
$project_name 是一个 $project_type 项目，基于 $architecture 架构设计。

## 技术栈
$tech_stack

## 项目状态
项目已完成初始化，等待功能开发。

## 下一步行动
使用 /dd:feature-add 添加第一个核心功能。
EOF
    echo "  ✅ project.md (默认模板)"
fi

# 生成 tech-stack.md - 使用AI内容或回退到模板  
if validate_content "$tech_content" >/dev/null 2>&1; then
    # 使用 AI 生成的内容
    cat > .claude/context/tech-stack.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
---

$tech_content
EOF
    echo "  ✅ tech-stack.md (AI生成内容)"
else
    # 使用基础模板
    cat > .claude/context/tech-stack.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
---

# 技术栈信息

## 主要技术选择
$tech_stack

## 架构模式
$architecture

## 开发环境
- 版本控制: Git
- 代码规范: 待建立
- 测试策略: 待规划

## 下一步行动
使用 /dd:chat 深入讨论具体技术实现方案。
EOF
    echo "  ✅ tech-stack.md (默认模板)"
fi

# 生成 architecture.md - 使用AI内容或回退到模板
if validate_content "$architecture_content" >/dev/null 2>&1; then
    # 使用 AI 生成的内容
    cat > .claude/context/architecture.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
architecture_pattern: $architecture
---

$architecture_content
EOF
    echo "  ✅ architecture.md (AI生成内容)"
else
    # 使用基础模板
    cat > .claude/context/architecture.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
architecture_pattern: $architecture
---

# 项目架构

## 架构模式
采用 $architecture 架构模式。

## 技术栈
$tech_stack

## 核心原则
- 模块化设计
- 低耦合高内聚  
- 可扩展性优先
- 安全性内置

## 架构决策
架构详细设计待深入分析后确定。

## 下一步行动
使用 /dd:chat 深入讨论架构设计方案。
EOF
    echo "  ✅ architecture.md (默认模板)"
fi

# 生成 requirements.md - 使用AI内容或回退到模板
if validate_content "$requirements_content" >/dev/null 2>&1; then
    # 使用 AI 生成的内容
    cat > .claude/context/requirements.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
project_type: $project_type
---

$requirements_content
EOF
    echo "  ✅ requirements.md (AI生成内容)"
else
    # 使用基础模板
    cat > .claude/context/requirements.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
version: 1.0.0
project_type: $project_type
---

# 项目需求文档

## 项目概述
$project_name 是一个 $project_type 项目。

## 技术要求
- 主要技术栈: $tech_stack
- 架构模式: $architecture

## 功能需求
- 待通过 /dd:feature-add 添加具体功能需求

## 非功能需求
- 性能要求: 待明确
- 安全要求: 待明确  
- 可维护性: 遵循最佳实践
- 可扩展性: 支持后续功能扩展

## 约束条件
- 技术约束: 基于选定的技术栈
- 时间约束: 待确定
- 资源约束: 待确定

## 下一步行动
使用 /dd:feature-add 添加具体功能需求，完善需求文档。
EOF
    echo "  ✅ requirements.md (默认模板)"
fi

# 生成 current-status.md - 使用AI内容或回退到模板
if validate_content "$status_content" >/dev/null 2>&1; then
    # 使用 AI 生成的内容
    cat > .claude/context/current-status.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
project_phase: 项目初始化完成
overall_progress: 10%
---

$status_content
EOF
    echo "  ✅ current-status.md (AI生成内容)"
else
    # 使用基础模板
    cat > .claude/context/current-status.md << EOF
---
last_updated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
project_phase: 项目初始化完成
overall_progress: 10%
---

# 当前项目状态

## 项目阶段
**当前阶段**: 项目初始化完成
- [x] 项目初始化和配置
- [x] DD工作流系统配置  
- [x] 基础架构规划
- [ ] 核心功能开发

## 下一步行动
1. 使用 /dd:feature-add 添加第一个核心功能
2. 使用 /dd:chat 深入讨论具体需求
3. 开始功能设计和开发工作

## DD工作流状态
- ✅ 系统已配置完成
- ✅ 项目上下文已建立
- 🚀 准备开始功能开发
EOF
    echo "  ✅ current-status.md (默认模板)"
fi

echo ""

## 3. 环境准备和状态更新

echo "🧹 准备项目环境..."

# 确保功能目录存在且为空
rm -rf .claude/features/* 2>/dev/null || true
mkdir -p .claude/features
echo "  ✅ 功能目录已清理"

# 验证DD系统配置文件
if [ ! -f ".claude/CLAUDE.md" ]; then
    echo "❌ 错误: DD系统配置文件不存在"
    exit 1
fi

# 检查根目录CLAUDE.md状态
if [ -f "CLAUDE.md" ]; then
    existing_lines=$(wc -l < CLAUDE.md)
    echo "  🔍 根目录现有 CLAUDE.md: $existing_lines 行"
else
    echo "  📄 根目录无 CLAUDE.md 文件"
fi

dd_lines=$(wc -l < .claude/CLAUDE.md)
echo "  📊 DD配置文件: $dd_lines 行"
echo "  ⏭️  配置文件合并将在下一步执行"

echo ""

## 4. 执行完成回调

echo "🎉 DD项目初始化后处理完成！"
echo ""
echo "📋 项目信息:"
echo "  • 项目名称: $project_name"  
echo "  • 项目类型: $project_type"
echo "  • 技术栈: $tech_stack"
echo "  • 架构模式: $architecture"
echo ""

# 生成执行结果报告供AI读取
execution_result=$(cat << EOF
{
  "status": "success",
  "project": {
    "name": "$project_name",
    "type": "$project_type", 
    "tech_stack": "$tech_stack",
    "architecture": "$architecture"
  },
  "files_created": [
    ".claude/context/project.md",
    ".claude/context/tech-stack.md", 
    ".claude/context/architecture.md",
    ".claude/context/requirements.md",
    ".claude/context/current-status.md"
  ],
  "next_actions": [
    "/dd:chat - 项目深度咨询",
    "/dd:feature-add - 添加核心功能",
    "/dd:status - 查看项目状态"
  ]
}
EOF
)

# 临时保存结果供AI读取
echo "$execution_result" > .claude/temp/init-result.json 2>/dev/null || true

echo "✅ 项目上下文文件已生成"
echo "🚀 DD工作流系统已配置完成"
echo ""
echo "💡 建议下一步:"
echo "   /dd:chat - 讨论第一个核心功能"
echo "   /dd:feature-add <功能名> - 添加功能"

exit 0