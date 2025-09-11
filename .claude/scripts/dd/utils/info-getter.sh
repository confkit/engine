#!/bin/bash

# DD 通用信息获取工具
# 通过入参返回系统信息和项目文件内容

set -e

show_help() {
  echo "DD 通用信息获取工具"
  echo "===================="
  echo ""
  echo "用法: "
  echo "  $0 time                    # 本地时间 (兼容 mac/linux)"
  echo "  $0 datetime               # 详细日期时间"
  echo "  $0 project                # 项目介绍内容"
  echo "  $0 architecture           # 架构文件内容"
  echo "  $0 tech-stack             # 技术栈文件内容"
  echo "  $0 requirements           # 需求文件内容"
  echo "  $0 context <filename>     # 获取指定上下文文件"
  echo "  $0 all-context            # 所有可用的上下文文件"
  echo "  $0 system                 # 系统基础信息"
  echo "  $0 all                    # 输出所有信息"
}

get_local_time() {
  # 兼容 macOS 和 Linux 的时间获取
  if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    date "+%Y-%m-%d %H:%M:%S"
  else
    # Linux
    date "+%Y-%m-%d %H:%M:%S"
  fi
}

get_detailed_datetime() {
  local current_time
  local timezone
  
  if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    current_time=$(date "+%Y-%m-%d %H:%M:%S")
    timezone=$(date "+%Z")
  else
    # Linux
    current_time=$(date "+%Y-%m-%d %H:%M:%S")
    timezone=$(date "+%Z")
  fi
  
  echo "当前时间: $current_time ($timezone)"
  echo "时间戳: $(date +%s)"
  echo "ISO格式: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
}

get_project_info() {
  local project_file=".claude/context/project.md"
  if [ -f "$project_file" ]; then
    echo "=== 项目介绍 ==="
    cat "$project_file"
  else
    echo "错误: 项目文件不存在 ($project_file)"
    return 1
  fi
}

get_architecture_info() {
  local arch_file=".claude/context/architecture.md"
  if [ -f "$arch_file" ]; then
    echo "=== 架构设计 ==="
    cat "$arch_file"
  else
    echo "错误: 架构文件不存在 ($arch_file)"
    return 1
  fi
}

get_tech_stack_info() {
  local tech_file=".claude/context/tech-stack.md"
  if [ -f "$tech_file" ]; then
    echo "=== 技术栈 ==="
    cat "$tech_file"
  else
    echo "错误: 技术栈文件不存在 ($tech_file)"
    return 1
  fi
}

get_requirements_info() {
  local req_file=".claude/context/requirements.md"
  if [ -f "$req_file" ]; then
    echo "=== 需求文档 ==="
    cat "$req_file"
  else
    echo "错误: 需求文件不存在 ($req_file)"
    return 1
  fi
}

get_context_file() {
  local filename="$1"
  local context_file=".claude/context/$filename"
  
  # 如果没有 .md 后缀，自动添加
  if [[ "$filename" != *.md ]]; then
    context_file="${context_file}.md"
  fi
  
  if [ -f "$context_file" ]; then
    echo "=== 上下文文件: $filename ==="
    cat "$context_file"
  else
    echo "错误: 上下文文件不存在 ($context_file)"
    return 1
  fi
}

list_all_context() {
  local context_dir=".claude/context"
  
  echo "=== 可用的上下文文件 ==="
  if [ -d "$context_dir" ]; then
    echo "目录: $context_dir"
    echo ""
    
    local found_files=false
    for file in "$context_dir"/*.md; do
      if [ -f "$file" ]; then
        local basename_file=$(basename "$file")
        local filesize=$(wc -c < "$file" 2>/dev/null || echo "0")
        local lines=$(wc -l < "$file" 2>/dev/null || echo "0")
        echo "  📄 $basename_file (${filesize}字节, ${lines}行)"
        found_files=true
      fi
    done
    
    if [ "$found_files" = false ]; then
      echo "  (没有找到 .md 文件)"
    fi
    
    echo ""
    echo "用法示例:"
    echo "  $0 context project          # 获取 project.md"
    echo "  $0 context architecture     # 获取 architecture.md"
    echo "  $0 context tech-stack       # 获取 tech-stack.md"
  else
    echo "错误: 上下文目录不存在 ($context_dir)"
    return 1
  fi
}

get_system_info() {
  echo "=== 系统信息 ==="
  echo "操作系统: $(uname -s)"
  echo "系统版本: $(uname -r)"
  echo "机器架构: $(uname -m)"
  echo "当前用户: $(whoami)"
  echo "当前目录: $(pwd)"
  echo "Shell: $SHELL"
  
  if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "系统类型: macOS"
    if command -v sw_vers >/dev/null 2>&1; then
      echo "macOS版本: $(sw_vers -productVersion)"
    fi
  elif [[ "$OSTYPE" == "linux"* ]]; then
    echo "系统类型: Linux"
    if [ -f /etc/os-release ]; then
      echo "发行版: $(grep PRETTY_NAME /etc/os-release | cut -d '"' -f 2)"
    fi
  fi
}

show_all_info() {
  echo "==============================================="
  echo "DD 系统信息完整报告"
  echo "生成时间: $(get_local_time)"
  echo "==============================================="
  echo ""
  
  # 系统信息
  get_system_info
  echo ""
  
  # 详细时间信息
  get_detailed_datetime
  echo ""
  
  # 项目上下文文件
  echo "=== 项目上下文文件 ==="
  
  if get_project_info >/dev/null 2>&1; then
    get_project_info
    echo ""
  fi
  
  if get_architecture_info >/dev/null 2>&1; then
    get_architecture_info
    echo ""
  fi
  
  if get_tech_stack_info >/dev/null 2>&1; then
    get_tech_stack_info
    echo ""
  fi
  
  if get_requirements_info >/dev/null 2>&1; then
    get_requirements_info
    echo ""
  fi
  
  # 列出所有可用文件
  list_all_context
}

main() {
  case "${1:-}" in
    "time")
      get_local_time
      ;;
    "datetime")
      get_detailed_datetime
      ;;
    "project")
      get_project_info
      ;;
    "architecture")
      get_architecture_info
      ;;
    "tech-stack")
      get_tech_stack_info
      ;;
    "requirements")
      get_requirements_info
      ;;
    "context")
      if [ -z "$2" ]; then
        echo "错误: 缺少文件名参数"
        echo "用法: $0 context <filename>"
        exit 1
      fi
      get_context_file "$2"
      ;;
    "all-context")
      list_all_context
      ;;
    "system")
      get_system_info
      ;;
    "all")
      show_all_info
      ;;
    "--help"|"-h"|"help"|"")
      show_help
      ;;
    *)
      echo "错误: 未知命令 '$1'"
      echo ""
      show_help
      exit 1
      ;;
  esac
}

main "$@"