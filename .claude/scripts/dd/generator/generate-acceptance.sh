#!/bin/bash

# DD 功能验收文档生成器
# 基于 JSON 数据生成 acceptance.md 功能验收文档

set -e

# 解析参数
FEATURE_NAME="$1"
acceptance_data="$2"

if [ -z "$FEATURE_NAME" ]; then
    echo "ERROR: Missing feature name parameter"
    echo "用法: bash generate-acceptance.sh '<feature_name>' '<json_data>'"
    exit 1
fi

if [ -z "$acceptance_data" ] || [ "$acceptance_data" = "null" ]; then
    echo "ERROR: Missing JSON data parameter"
    echo "用法: bash generate-acceptance.sh '<feature_name>' '<json_data>'"
    exit 1
fi

FEATURE_DIR=".claude/features/$FEATURE_NAME"
ACCEPTANCE_FILE="$FEATURE_DIR/acceptance.md"

echo "=== FEATURE_ADD_ACCEPTANCE ==="
echo "FEATURE_NAME: $FEATURE_NAME"
echo "FEATURE_DIR: $FEATURE_DIR"
echo ""

# 确保功能目录存在
mkdir -p "$FEATURE_DIR"

# 从 JSON 中提取数据
functional_requirements=$(echo "$acceptance_data" | jq -r '.functional_requirements // "功能验收点待定义"')
performance_requirements=$(echo "$acceptance_data" | jq -r '.performance_requirements // "性能验收标准待定义"')
security_requirements=$(echo "$acceptance_data" | jq -r '.security_requirements // "安全验收标准待定义"')
other_requirements=$(echo "$acceptance_data" | jq -r '.other_requirements // "其他验收要求待定义"')
acceptance_criteria=$(echo "$acceptance_data" | jq -r '.acceptance_criteria // "详细验收条件待定义"')

# 生成验收标准文档
cat > "$ACCEPTANCE_FILE" << EOF
---
feature_name: $FEATURE_NAME
document_type: 验收标准
---

# 功能验收标准: $FEATURE_NAME

## 功能点验收
$functional_requirements

## 性能验收标准
$performance_requirements

## 安全性验收标准
$security_requirements

## 其他验收要求
$other_requirements

## 详细验收条件
$acceptance_criteria

## 验收流程

### 验收前准备
- [ ] 功能开发完成
- [ ] 单元测试通过
- [ ] 代码审查完成
- [ ] 部署到测试环境

### 验收执行
- [ ] 功能点逐项验收
- [ ] 性能指标验证
- [ ] 安全性检查
- [ ] 其他要求验证

### 验收完成
- [ ] 所有验收点通过
- [ ] 问题修复确认
- [ ] 验收报告生成
- [ ] 功能发布准备
EOF

echo "✅ 验收标准文档已生成: $ACCEPTANCE_FILE"
echo ""