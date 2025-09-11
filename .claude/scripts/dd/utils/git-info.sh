#!/bin/bash

# DD Git 信息查询脚本
# 提供精简的 Git 状态信息供 hooks 使用

set -e

show_help() {
  echo "DD Git 信息查询工具"
  echo "=================="
  echo ""
  echo "用法: "
  echo "  $0 branch              # 当前分支名"
  echo "  $0 status              # 工作区状态"
  echo "  $0 remote              # 远程分支状态"
  echo "  $0 clean               # 工作区是否干净"
  echo "  $0 feature <name>      # 检查是否在指定功能分支"
  echo "  $0 updates             # 检查是否有远程更新"
  echo "  $0 all                 # 输出所有信息"
}

get_current_branch() {
  git branch --show-current 2>/dev/null || echo ""
}

get_work_status() {
  if [ -z "$(git status --porcelain 2>/dev/null)" ]; then
    echo "clean"
  else
    echo "dirty"
  fi
}

get_remote_status() {
  local current_branch=$(get_current_branch)
  if [ -z "$current_branch" ]; then
    echo "no_branch"
    return
  fi
  
  # 检查是否有远程追踪分支
  if git rev-parse --verify "origin/$current_branch" >/dev/null 2>&1; then
    local ahead=$(git rev-list --count HEAD...origin/$current_branch 2>/dev/null || echo "0")
    local behind=$(git rev-list --count origin/$current_branch...HEAD 2>/dev/null || echo "0")
    
    if [ "$ahead" -eq 0 ] && [ "$behind" -eq 0 ]; then
      echo "synced"
    elif [ "$ahead" -gt 0 ] && [ "$behind" -eq 0 ]; then
      echo "ahead:$ahead"
    elif [ "$ahead" -eq 0 ] && [ "$behind" -gt 0 ]; then
      echo "behind:$behind"
    else
      echo "diverged:$ahead:$behind"
    fi
  else
    echo "no_remote"
  fi
}

check_is_clean() {
  if [ "$(get_work_status)" = "clean" ]; then
    echo "true"
  else
    echo "false"
  fi
}

check_feature_branch() {
  local feature_name="$1"
  local current_branch=$(get_current_branch)
  local expected_branch="feature/$feature_name"
  
  if [ "$current_branch" = "$expected_branch" ]; then
    echo "true"
  else
    echo "false"
    echo "current: $current_branch"
    echo "expected: $expected_branch"
  fi
}

check_remote_updates() {
  # 获取远程更新
  git fetch >/dev/null 2>&1 || {
    echo "fetch_failed"
    return
  }
  
  local current_branch=$(get_current_branch)
  if [ -z "$current_branch" ]; then
    echo "no_branch"
    return
  fi
  
  # 检查是否有远程分支
  if ! git rev-parse --verify "origin/$current_branch" >/dev/null 2>&1; then
    echo "no_remote_branch"
    return
  fi
  
  # 检查是否有待拉取的更新
  local behind=$(git rev-list --count HEAD..origin/$current_branch 2>/dev/null || echo "0")
  if [ "$behind" -gt 0 ]; then
    echo "updates_available:$behind"
  else
    echo "up_to_date"
  fi
}

show_all_info() {
  echo "=== GIT_INFO_ALL ==="
  echo "CURRENT_BRANCH: $(get_current_branch)"
  echo "WORK_STATUS: $(get_work_status)"
  echo "REMOTE_STATUS: $(get_remote_status)"
  echo "IS_CLEAN: $(check_is_clean)"
  echo "REMOTE_UPDATES: $(check_remote_updates)"
  echo ""
  
  # 显示简要的状态信息
  echo "=== WORK_TREE_STATUS ==="
  git status --porcelain 2>/dev/null | head -10
  echo ""
  
  echo "=== RECENT_COMMITS ==="
  git log --oneline -5 2>/dev/null || echo "No commits"
}

main() {
  case "${1:-}" in
    "branch")
      get_current_branch
      ;;
    "status")
      get_work_status
      ;;
    "remote")
      get_remote_status
      ;;
    "clean")
      check_is_clean
      ;;
    "feature")
      if [ -z "$2" ]; then
        echo "ERROR: Missing feature name parameter"
        exit 1
      fi
      check_feature_branch "$2"
      ;;
    "updates")
      check_remote_updates
      ;;
    "all")
      show_all_info
      ;;
    "--help"|"-h"|"help"|"")
      show_help
      ;;
    *)
      echo "ERROR: Unknown command '$1'"
      show_help
      exit 1
      ;;
  esac
}

main "$@"