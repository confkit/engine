#!/bin/bash

# DD Chat 持久上下文加载脚本
# 读取并整合所有项目上下文文件, 为智能对话提供完整的项目感知能力

set -e

echo "=== DD 项目上下文 ==="

echo "\n📄 项目信息"
if [ -f ".claude/context/project.md" ]; then
    cat ".claude/context/project.md"
else
    echo "project.md 未找到"
fi

echo "\n🏗️ 架构信息"
if [ -f ".claude/context/architecture.md" ]; then
    cat ".claude/context/architecture.md"
else
    echo "architecture.md 未找到"
fi

echo "\n⚙️ 技术栈"
if [ -f ".claude/context/tech-stack.md" ]; then
    cat ".claude/context/tech-stack.md"
else
    echo "tech-stack.md 未找到"
fi

echo "\n📊 项目状态"
if [ -f ".claude/context/current-status.md" ]; then
    cat ".claude/context/current-status.md"
else
    echo "current-status.md 未找到"
fi

echo "\n📋 功能概览"
if [ -d ".claude/features" ]; then
    feature_count=$(find ".claude/features" -maxdepth 1 -type d | grep -v "^\.claude/features$" | wc -l)
    echo "功能数量: $feature_count"
    
    if [ "$feature_count" -gt 0 ]; then
        find ".claude/features" -maxdepth 1 -type d ! -path ".claude/features" | while read feature_dir; do
            feature_name=$(basename "$feature_dir")
            echo "\n- $feature_name"
            if [ -f "$feature_dir/overview.md" ]; then
                grep -E "^(name|status|progress):" "$feature_dir/overview.md" 2>/dev/null | head -3
            fi
            
            if [ -d "$feature_dir/issues" ]; then
                issue_count=$(find "$feature_dir/issues" -name "*.md" 2>/dev/null | wc -l)
                echo "  议题数: $issue_count"
            fi
        done
    fi
else
    echo "无功能目录"
fi

echo "\n📋 会话历史"
if [ -d ".claude/context/session" ]; then
    session_count=$(find ".claude/context/session" -name "*.md" 2>/dev/null | wc -l)
    echo "会话数量: $session_count"
else
    echo "session 目录未找到"
fi

echo "\n⚙️ Claude 配置"
if [ -f "CLAUDE.md" ]; then
    echo "DD 命令: $(grep -c '/dd:' CLAUDE.md) 个"
else
    echo "CLAUDE.md 未找到"
fi

echo "\n📦 DD 系统"
if [ -d ".claude" ]; then
    echo "目录: ✅  命令: $(find .claude/commands -name "*.md" 2>/dev/null | wc -l)  脚本: $(find .claude/scripts -name "*.sh" 2>/dev/null | wc -l)  智能体: $(find .claude/agents -name "*.md" 2>/dev/null | wc -l)"
else
    echo ".claude 目录缺失"
fi

echo "\n✨ 上下文加载完成 - 准备智能对话"