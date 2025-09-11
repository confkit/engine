#!/bin/bash

# DD 议题文档生成器
# 动态生成单个议题的详细文档

set -e

FEATURE_NAME="$1"
ISSUE_ID="$2"
ISSUE_DATA="$3"  # JSON格式的议题数据
OVERWRITE="${4:-false}"  # 可选参数, 默认为false

if [ -z "$FEATURE_NAME" ] || [ -z "$ISSUE_ID" ] || [ -z "$ISSUE_DATA" ]; then
    echo "ERROR: Missing required parameters"
    echo "Usage: $0 <feature_name> <issue_id> <issue_data_json> [overwrite]"
    echo "  overwrite: true|false (default: false)"
    exit 1
fi

FEATURE_DIR=".claude/features/$FEATURE_NAME"
ISSUES_DIR="$FEATURE_DIR/issues"
ISSUE_FILE="$ISSUES_DIR/$ISSUE_ID.md"

echo "=== ISSUE_DETAIL_GENERATION ==="
echo "FEATURE_NAME: $FEATURE_NAME"
echo "ISSUE_ID: $ISSUE_ID"
echo "ISSUE_FILE: $ISSUE_FILE"
echo "OVERWRITE: $OVERWRITE"
echo ""

# 检查文件是否存在, 是否允许覆盖
if [ -f "$ISSUE_FILE" ] && [ "$OVERWRITE" != "true" ]; then
    echo "ERROR: Issue file already exists: $ISSUE_FILE"
    echo "Use overwrite=true to replace existing file"
    exit 1
fi

# 确保目录存在
mkdir -p "$ISSUES_DIR"

# JSON解析函数
parse_json_field() {
    local json="$1"
    local field="$2"
    local default="$3"
    
    local value=$(echo "$json" | grep -o "\"$field\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" | sed "s/\"$field\"[[:space:]]*:[[:space:]]*\"\([^\"]*\)\"/\1/" || echo "$default")
    echo "$value"
}

parse_json_array() {
    local json="$1"
    local field="$2"
    
    echo "$json" | grep -o "\"$field\"[[:space:]]*:[[:space:]]*\[[^\]]*\]" | sed "s/\"$field\"[[:space:]]*:[[:space:]]*\[\([^\]]*\)\]/\1/" | sed 's/"//g'
}

# 解析议题数据
ISSUE_NAME=$(parse_json_field "$ISSUE_DATA" "name" "议题$ISSUE_ID")
ISSUE_GOAL=$(parse_json_field "$ISSUE_DATA" "goal" "")
ISSUE_POINTS=$(parse_json_field "$ISSUE_DATA" "implementation_points" "")
ISSUE_DETAILS=$(parse_json_field "$ISSUE_DATA" "technical_details" "")
ISSUE_DEPENDENCIES=$(parse_json_array "$ISSUE_DATA" "dependencies")
ISSUE_TODOS=$(parse_json_array "$ISSUE_DATA" "todos")
ISSUE_ACCEPTANCE=$(parse_json_array "$ISSUE_DATA" "acceptance_criteria")

echo "=== PARSED_ISSUE_DATA ==="
echo "ISSUE_NAME: $ISSUE_NAME"
echo ""

# 生成议题文件内容
cat > "$ISSUE_FILE" << EOF
---
name: $ISSUE_NAME
feature: $FEATURE_NAME
status: 未开始
progress: 0
dependencies: [$ISSUE_DEPENDENCIES]
---

# $ISSUE_NAME

## 议题目标
$ISSUE_GOAL

## 实现要点
EOF

# 添加实现要点（如果有的话, 按行分割）
if [ -n "$ISSUE_POINTS" ]; then
    echo "$ISSUE_POINTS" | sed 's/\\n/\n/g' | while IFS= read -r point; do
        if [ -n "$point" ]; then
            echo "- $point" >> "$ISSUE_FILE"
        fi
    done
else
    echo "- 待补充实现要点" >> "$ISSUE_FILE"
fi

cat >> "$ISSUE_FILE" << EOF

## 验收标准
EOF

# 添加验收标准列表
if [ -n "$ISSUE_ACCEPTANCE" ]; then
    echo "$ISSUE_ACCEPTANCE" | sed 's/,/\n/g' | while IFS= read -r criterion; do
        criterion=$(echo "$criterion" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//')
        if [ -n "$criterion" ]; then
            echo "- [ ] $criterion" >> "$ISSUE_FILE"
        fi
    done
else
    echo "- [ ] 功能实现完成" >> "$ISSUE_FILE"
    echo "- [ ] 单元测试通过" >> "$ISSUE_FILE"
    echo "- [ ] 代码审查通过" >> "$ISSUE_FILE"
fi

cat >> "$ISSUE_FILE" << EOF

## 技术细节
EOF

# 添加技术细节（如果有的话, 按行分割）
if [ -n "$ISSUE_DETAILS" ]; then
    echo "$ISSUE_DETAILS" | sed 's/\\n/\n/g' >> "$ISSUE_FILE"
else
    echo "待补充技术实现细节" >> "$ISSUE_FILE"
fi

cat >> "$ISSUE_FILE" << EOF

## Todo 列表
EOF

# 添加Todo列表
if [ -n "$ISSUE_TODOS" ]; then
    echo "$ISSUE_TODOS" | sed 's/,/\n/g' | while IFS= read -r todo; do
        todo=$(echo "$todo" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//')
        if [ -n "$todo" ]; then
            echo "- [ ] $todo" >> "$ISSUE_FILE"
        fi
    done
else
    echo "- [ ] 分析需求和设计方案" >> "$ISSUE_FILE"
    echo "- [ ] 编写核心实现代码" >> "$ISSUE_FILE"
    echo "- [ ] 编写单元测试" >> "$ISSUE_FILE"
    echo "- [ ] 进行代码审查" >> "$ISSUE_FILE"
    echo "- [ ] 更新相关文档" >> "$ISSUE_FILE"
fi

echo ""
echo "=== GENERATION_RESULT ==="
echo "ISSUE_FILE_CREATED: $ISSUE_FILE"
echo "ISSUE_FILE_SIZE: $(wc -c < "$ISSUE_FILE") bytes"
echo "ISSUE_LINES: $(wc -l < "$ISSUE_FILE") lines"

# 验证依赖关系（如果有的话）
if [ -n "$ISSUE_DEPENDENCIES" ] && [ "$ISSUE_DEPENDENCIES" != "[]" ]; then
    echo "ISSUE_DEPENDENCIES: $ISSUE_DEPENDENCIES"
    echo "DEPENDENCY_CHECK: $(echo "$ISSUE_DEPENDENCIES" | wc -w) dependencies found"
else
    echo "ISSUE_DEPENDENCIES: none"
fi

echo ""
echo "✅ ISSUE detail generated: $ISSUE_ID.md for feature $FEATURE_NAME"