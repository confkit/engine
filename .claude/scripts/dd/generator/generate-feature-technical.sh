#!/bin/bash

# DD 技术方案文档生成器
# 基于 JSON 数据生成 technical.md 技术设计文档

set -e

# 解析参数
FEATURE_NAME="$1"
technical_data="$2"

if [ -z "$FEATURE_NAME" ]; then
    echo "ERROR: Missing feature name parameter"
    echo "用法: bash generate-feature-technical.sh '<feature_name>' '<json_data>'"
    exit 1
fi

if [ -z "$technical_data" ] || [ "$technical_data" = "null" ]; then
    echo "ERROR: Missing JSON data parameter"
    echo "用法: bash generate-feature-technical.sh '<feature_name>' '<json_data>'"
    exit 1
fi

FEATURE_DIR=".claude/features/$FEATURE_NAME"
TECHNICAL_FILE="$FEATURE_DIR/technical.md"

echo "=== FEATURE_ADD_TECHNICAL ==="
echo "FEATURE_NAME: $FEATURE_NAME"
echo "TECHNICAL_FILE: $TECHNICAL_FILE"
echo ""

# 确保功能目录存在
mkdir -p "$FEATURE_DIR"

# 从 JSON 中提取数据
architecture_design=$(echo "$technical_data" | jq -r '.architecture_design // "架构设计待完善"')
data_models=$(echo "$technical_data" | jq -r '.data_models // "数据模型待设计"')
api_design=$(echo "$technical_data" | jq -r '.api_design // "API设计待定义"')
database_design=$(echo "$technical_data" | jq -r '.database_design // "数据库设计待规划"')
security_considerations=$(echo "$technical_data" | jq -r '.security_considerations // "安全考虑待评估"')
performance_requirements=$(echo "$technical_data" | jq -r '.performance_requirements // "性能要求待明确"')
tech_stack=$(echo "$technical_data" | jq -r '.tech_stack // "技术栈待确定"')
external_integrations=$(echo "$technical_data" | jq -r '.external_integrations // "无外部集成"')
deployment_strategy=$(echo "$technical_data" | jq -r '.deployment_strategy // "部署策略待规划"')

# 生成技术方案文档
cat > "$TECHNICAL_FILE" << EOF
---
feature_name: $FEATURE_NAME
document_type: technical_design
version: 1.0.0
---

# 技术设计: $FEATURE_NAME

## 架构设计
$architecture_design

## 数据模型
$data_models

## API 设计
$api_design

## 数据库设计
$database_design

## 技术栈选择
$tech_stack

## 外部集成
$external_integrations

## 安全考虑
$security_considerations

## 性能要求
$performance_requirements

## 部署策略
$deployment_strategy

## 技术风险
### 已识别风险
- 待评估具体技术风险

### 风险缓解措施
- 待制定相应的缓解策略

## 开发指南
### 开发环境要求
- 待明确开发环境配置

### 编码规范
- 遵循项目既定的编码规范
- 保持代码质量和一致性

## 监控和日志
### 性能监控
- 待设计性能监控指标

### 日志策略  
- 待确定日志记录策略

## 更新历史
- $(date +"%Y-%m-%d"): 技术设计文档创建
EOF

echo "✅ 技术方案文档已生成: $TECHNICAL_FILE"
echo ""