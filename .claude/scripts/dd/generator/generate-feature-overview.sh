#!/bin/bash

# DD 功能描述文档生成器
# 基于 JSON 数据生成 overview.md 功能描述文档

set -e

# 解析参数
FEATURE_NAME="$1"
feature_data="$2"

if [ -z "$FEATURE_NAME" ]; then
    echo "ERROR: Missing feature name parameter"
    echo "用法: bash generate-feature-overview.sh '<feature_name>' '<json_data>'"
    exit 1
fi

if [ -z "$feature_data" ] || [ "$feature_data" = "null" ]; then
    echo "ERROR: Missing JSON data parameter"
    echo "用法: bash generate-feature-overview.sh '<feature_name>' '<json_data>'"
    exit 1
fi

FEATURE_DIR=".claude/features/$FEATURE_NAME"
FEATURE_FILE="$FEATURE_DIR/overview.md"

echo "=== FEATURE_ADD_FEATURE ==="
echo "FEATURE_NAME: $FEATURE_NAME"
echo "FEATURE_DIR: $FEATURE_DIR"
echo ""

# 确保功能目录存在
mkdir -p "$FEATURE_DIR"

# 从 JSON 中提取数据
goal=$(echo "$feature_data" | jq -r '.goal // "功能目标待定义"')
user_value=$(echo "$feature_data" | jq -r '.user_value // "用户价值待明确"')
core_features=$(echo "$feature_data" | jq -r '.core_features // "核心功能待设计"')
feature_boundary_include=$(echo "$feature_data" | jq -r '.feature_boundary_include // "功能边界待定义"')
feature_boundary_exclude=$(echo "$feature_data" | jq -r '.feature_boundary_exclude // "排除边界待明确"')
use_scenarios=$(echo "$feature_data" | jq -r '.use_scenarios // "使用场景待描述"')
dependencies=$(echo "$feature_data" | jq -r '.dependencies // "无特殊依赖"')

# 生成精简的功能描述文档
cat > "$FEATURE_FILE" << EOF
---
name: $FEATURE_NAME
status: 设计中
progress: 0%
issues_total: 0
issues_completed: 0
---

# 功能: $FEATURE_NAME

## issues

## 功能目标
$goal

## 用户价值
$user_value

## 核心功能
$core_features

## 功能边界

### 包含的功能
$feature_boundary_include

### 不包含的功能  
$feature_boundary_exclude

## 使用场景
$use_scenarios
EOF

echo "✅ 功能描述文档已生成: $FEATURE_FILE"
echo ""