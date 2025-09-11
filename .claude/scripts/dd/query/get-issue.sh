#!/bin/bash

# DD 议题信息读取脚本
# 读取议题文档的完整信息，包括状态、进度和详细内容
# 默认获取全部内容，提供 --status-only 参数仅查询状态

set -e

ISSUE_ID=""
FEATURE_NAME=""
ISSUE_NUMBER=""
ISSUE_FILE=""
STATUS_ONLY=false

parse_issue_id() {
  if [ -z "$1" ]; then
    echo "ERROR: Missing issue parameter"
    exit 1
  fi
  
  # 解析功能名和议题编号
  if [[ "$1" == *":"* ]]; then
    FEATURE_NAME="${1%:*}"
    ISSUE_NUMBER="${1#*:}"
  else
    echo "ERROR: Invalid issue format. Expected: <feature>:<issue_id>"
    exit 1
  fi
  
  ISSUE_ID="$1"
  ISSUE_FILE=".claude/features/$FEATURE_NAME/issues/$ISSUE_NUMBER.md"
}

read_issue_status() {
  echo "=== ISSUE_STATUS_READ ==="
  echo "ISSUE_ID: $ISSUE_ID"
  echo "FEATURE_NAME: $FEATURE_NAME"
  echo "ISSUE_NUMBER: $ISSUE_NUMBER"
  echo "ISSUE_FILE: $ISSUE_FILE"
  echo ""
  
  # 检查议题文件是否存在
  if [ ! -f "$ISSUE_FILE" ]; then
    echo "ERROR: Issue file does not exist: $ISSUE_FILE"
    exit 1
  fi
  
  echo "=== ISSUE_METADATA ==="
  # 读取 YAML frontmatter 中的关键信息
  name=$(grep "^name:" "$ISSUE_FILE" 2>/dev/null | sed 's/^name: *//' || echo "")
  status=$(grep "^status:" "$ISSUE_FILE" 2>/dev/null | sed 's/^status: *//' || echo "未开始")
  progress=$(grep "^progress:" "$ISSUE_FILE" 2>/dev/null | sed 's/^progress: *//' || echo "0")
  feature=$(grep "^feature:" "$ISSUE_FILE" 2>/dev/null | sed 's/^feature: *//' || echo "")
  dependencies=$(grep "^dependencies:" "$ISSUE_FILE" 2>/dev/null | sed 's/^dependencies: *//' || echo "[]")
  
  echo "NAME: $name"
  echo "STATUS: $status"
  echo "PROGRESS: $progress"
  echo "FEATURE: $feature"
  echo "DEPENDENCIES: $dependencies"
  echo ""
  
  echo "=== TODO_ANALYSIS ==="
  # 分析 TODO 项目完成情况
  total_todos=$(grep -c "^- \[" "$ISSUE_FILE" 2>/dev/null || echo "0")
  completed_todos=$(grep -c "^- \[x\]" "$ISSUE_FILE" 2>/dev/null || echo "0")
  
  echo "TOTAL_TODOS: $total_todos"
  echo "COMPLETED_TODOS: $completed_todos"
  
  if [ "$total_todos" -gt 0 ]; then
    todo_progress=$(echo "scale=0; $completed_todos * 100 / $total_todos" | bc)
    echo "TODO_PROGRESS: ${todo_progress}%"
  else
    echo "TODO_PROGRESS: 0%"
  fi
  echo ""
  
  echo "=== SESSION_CONTEXT ==="
  # 检查会话上下文文件
  session_file=".claude/context/session/$ISSUE_NUMBER.md"
  if [ -f "$session_file" ]; then
    echo "SESSION_FILE: EXISTS"
    echo "LAST_MODIFIED: $(stat -f "%Sm" -t "%Y-%m-%d %H:%M:%S" "$session_file")"
  else
    echo "SESSION_FILE: NOT_EXISTS"
  fi
  echo ""
  
  # 仅在非status-only模式下显示完整内容
  if [ "$STATUS_ONLY" = false ]; then
    echo "=== ISSUE_CONTENT ==="
    echo "--- FULL_CONTENT ---"
    cat "$ISSUE_FILE"
    echo ""
  fi
  
  if [ "$STATUS_ONLY" = true ]; then
    echo "✅ Issue status read completed: $ISSUE_ID"
  else
    echo "✅ Issue information read completed: $ISSUE_ID"
  fi
}

main() {
  local issue_param=""
  
  # 解析参数
  while [[ $# -gt 0 ]]; do
    case $1 in
      --status-only)
        STATUS_ONLY=true
        shift
        ;;
      --help|-h|help)
        echo "DD 议题信息读取工具"
        echo "用法: $0 [选项] <feature>:<issue_id>"
        echo ""
        echo "选项:"
        echo "  --status-only    仅显示状态信息，不显示完整内容"
        echo "  --help, -h       显示此帮助信息"
        echo ""
        echo "示例:"
        echo "  $0 用户认证系统:001              # 显示完整信息"
        echo "  $0 --status-only 用户认证系统:001 # 仅显示状态"
        exit 0
        ;;
      *)
        if [ -z "$issue_param" ]; then
          issue_param="$1"
        else
          echo "ERROR: 意外的参数: $1"
          exit 1
        fi
        shift
        ;;
    esac
  done
  
  if [ -z "$issue_param" ]; then
    echo "ERROR: Missing issue parameter"
    echo "用法: $0 [选项] <feature>:<issue_id>"
    echo "使用 --help 查看完整帮助"
    exit 1
  fi
  
  parse_issue_id "$issue_param"
  read_issue_status
}

main "$@"