#!/bin/bash

# DD 项目状态脚本
# 显示项目整体开发状态和进度统计

set -e

get_project_info() {
  if [ -f ".claude/context/project.md" ]; then
    PROJECT_NAME=$(grep "^name:" .claude/context/project.md 2>/dev/null | cut -d: -f2- | xargs)
    PROJECT_TYPE=$(grep "^type:" .claude/context/project.md 2>/dev/null | cut -d: -f2- | xargs)
    PROJECT_STATUS=$(grep "^status:" .claude/context/project.md 2>/dev/null | cut -d: -f2- | xargs)
  else
    PROJECT_NAME="未初始化项目"
    PROJECT_TYPE="未知"
    PROJECT_STATUS="未初始化"
  fi
}

calculate_progress() {
  echo "=== PROGRESS_CALCULATION ==="
  if [ -x ".claude/scripts/dd/utils/progress-calc.sh" ]; then
    echo "🔄 生成实时进度报告..."
    bash .claude/scripts/dd/utils/progress-calc.sh report
    echo ""
    
    echo "🔄 同步所有进度数据..."
    bash .claude/scripts/dd/utils/progress-calc.sh sync
    echo ""
  else
    echo "⚠️  进度计算工具不可用, 使用基础统计方法"
    calculate_basic_progress
  fi
}

calculate_basic_progress() {
  local total_features=0
  local completed_features=0
  local active_features=0
  local pending_features=0
  local total_issues=0
  local completed_issues=0
  local active_issues=0
  
  if [ -d ".claude/features" ]; then
    for feature_dir in .claude/features/*/; do
      if [ -d "$feature_dir" ]; then
        total_features=$((total_features + 1))
        
        # 读取功能状态
        local feature_status="未开始"
        if [ -f "$feature_dir/overview.md" ]; then
          feature_status=$(grep "^status:" "$feature_dir/overview.md" 2>/dev/null | cut -d: -f2- | xargs)
        fi
        
        case "$feature_status" in
          "已完成") completed_features=$((completed_features + 1)) ;;
          "开发中"|"测试中") active_features=$((active_features + 1)) ;;
          *) pending_features=$((pending_features + 1)) ;;
        esac
        
        # 统计议题
        if [ -d "$feature_dir/issues" ]; then
          for issue_file in "$feature_dir/issues"/*.md; do
            if [ -f "$issue_file" ]; then
              total_issues=$((total_issues + 1))
              local issue_status=$(grep "^status:" "$issue_file" 2>/dev/null | cut -d: -f2- | xargs)
              case "$issue_status" in
                "已完成") completed_issues=$((completed_issues + 1)) ;;
                "进行中") active_issues=$((active_issues + 1)) ;;
              esac
            fi
          done
        fi
      fi
    done
  fi
  
  # 计算百分比
  if [ $total_features -gt 0 ]; then
    FEATURE_PROGRESS=$((completed_features * 100 / total_features))
  else
    FEATURE_PROGRESS=0
  fi
  
  if [ $total_issues -gt 0 ]; then
    ISSUE_PROGRESS=$((completed_issues * 100 / total_issues))
  else
    ISSUE_PROGRESS=0
  fi
  
  # 导出变量供显示使用
  TOTAL_FEATURES=$total_features
  COMPLETED_FEATURES=$completed_features
  ACTIVE_FEATURES=$active_features
  PENDING_FEATURES=$pending_features
  TOTAL_ISSUES=$total_issues
  COMPLETED_ISSUES=$completed_issues
  ACTIVE_ISSUES=$active_issues
}

get_git_status() {
  if git rev-parse --git-dir > /dev/null 2>&1; then
    CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
    UNSTAGED_FILES=$(git status --porcelain 2>/dev/null | wc -l)
    UNPUSHED_COMMITS=$(git log --oneline @{u}.. 2>/dev/null | wc -l || echo "0")
  else
    CURRENT_BRANCH="非Git项目"
    UNSTAGED_FILES=0
    UNPUSHED_COMMITS=0
  fi
}

determine_health() {
  local health_score=0
  
  # 项目初始化状态
  [ "$PROJECT_STATUS" != "未初始化" ] && health_score=$((health_score + 20))
  
  # 功能开发进度
  [ $FEATURE_PROGRESS -ge 50 ] && health_score=$((health_score + 30))
  [ $FEATURE_PROGRESS -ge 80 ] && health_score=$((health_score + 20))
  
  # Git状态
  [ $UNSTAGED_FILES -eq 0 ] && health_score=$((health_score + 15))
  [ $UNPUSHED_COMMITS -eq 0 ] && health_score=$((health_score + 15))
  
  if [ $health_score -ge 80 ]; then
    HEALTH_STATUS="优秀"
    HEALTH_ICON="🟢"
  elif [ $health_score -ge 60 ]; then
    HEALTH_STATUS="良好"
    HEALTH_ICON="🟡"
  else
    HEALTH_STATUS="需要关注"
    HEALTH_ICON="🔴"
  fi
}

show_status_report() {
  
  echo "🎯 $PROJECT_NAME - 项目状态报告"
  echo "========================================"
  echo "📊 项目类型: $PROJECT_TYPE"
  echo "📈 项目状态: $PROJECT_STATUS"
  echo ""
  
  echo "📊 开发进度统计: "
  echo "  总功能数: $TOTAL_FEATURES 个"
  echo "  已完成: $COMPLETED_FEATURES 个 ($FEATURE_PROGRESS%)"
  echo "  开发中: $ACTIVE_FEATURES 个"
  echo "  未开始: $PENDING_FEATURES 个"
  echo ""
  echo "  总议题数: $TOTAL_ISSUES 个"
  echo "  已完成: $COMPLETED_ISSUES 个 ($ISSUE_PROGRESS%)"
  echo "  进行中: $ACTIVE_ISSUES 个"
  echo ""
  
  echo "🔄 活跃开发: "
  if [ $ACTIVE_FEATURES -gt 0 ]; then
    echo "  当前有 $ACTIVE_FEATURES 个功能在开发中"
    # 列出正在开发的功能
    for feature_dir in .claude/features/*/; do
      if [ -f "$feature_dir/overview.md" ]; then
        local status=$(grep "^status:" "$feature_dir/overview.md" 2>/dev/null | cut -d: -f2- | xargs)
        if [[ "$status" =~ ^(开发中|测试中)$ ]]; then
          local feature_name=$(basename "$feature_dir")
          echo "    • $feature_name ($status)"
        fi
      fi
    done
  else
    echo "  当前无活跃开发功能"
  fi
  echo ""
  
  echo "📋 Git 状态: "
  echo "  分支: $CURRENT_BRANCH"
  echo "  未提交文件: $UNSTAGED_FILES 个"
  echo "  未推送提交: $UNPUSHED_COMMITS 个"
  echo ""
  
  echo "🏥 系统健康度: $HEALTH_ICON $HEALTH_STATUS"
  echo ""
  
  # 提供建议
  if [ $TOTAL_FEATURES -eq 0 ]; then
    echo "💡 建议操作: "
    echo "   /dd:prd-decompose  - 开始需求拆解"
    echo "   /dd:feature-add    - 添加第一个功能"
  elif [ $ACTIVE_FEATURES -eq 0 ] && [ $COMPLETED_FEATURES -lt $TOTAL_FEATURES ]; then
    echo "💡 建议操作: "
    echo "   /dd:feature-start  - 开始功能开发"
  elif [ $UNSTAGED_FILES -gt 0 ]; then
    echo "💡 建议操作: "
    echo "   /dd:code-reflect   - 分析代码变更"
    echo "   🤖 获取 AI commit message 建议:"
    echo "   bash .claude/scripts/dd/utils/commit-message-helper.sh suggest"
    echo "   git add && git commit -m '<AI建议>' - 用户手动提交 (AI 禁止执行)"
  fi
}

main() {
  get_project_info
  calculate_progress
  
  echo "=== GIT_STATUS_DETAILED ==="
  if [ -x ".claude/scripts/dd/utils/git-check.sh" ]; then
    echo "🔍 执行详细Git状态检查..."
    bash .claude/scripts/dd/utils/git-check.sh full-check
    echo ""
  else
    echo "⚠️  Git检查工具不可用, 使用基础Git状态"
    get_git_status
  fi
  
  determine_health
  show_status_report
}

main "$@"