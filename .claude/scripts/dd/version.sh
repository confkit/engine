#!/bin/bash

# DD 版本信息脚本
# 显示 CCDD Helper 系统版本和组件状态

set -e

CCDD_VERSION="2.0.0"
BUILD_DATE="2024-01-15"
BUILD_TYPE="stable"

show_version() {
  echo "📚 CCDD Helper - Claude Code 深度开发工作流系统"
  echo "=============================================="
  echo "版本: $CCDD_VERSION"
  echo "发布日期: $BUILD_DATE"  
  echo "构建: $BUILD_TYPE"
  echo ""
}

check_components() {
  echo "🔧 组件状态检查: "
  
  # 检查智能体配置
  local agents_count=$(ls -1 .claude/agents/*.md 2>/dev/null | wc -l)
  if [ "$agents_count" -eq 5 ]; then
    echo "  ✅ 智能体配置完整 (5个)"
  else
    echo "  ⚠️ 智能体配置不完整 ($agents_count/5个)"
  fi
  
  # 检查规则系统
  local rules_count=$(ls -1 .claude/rules/*.md 2>/dev/null | wc -l)
  if [ "$rules_count" -eq 5 ]; then
    echo "  ✅ 规则系统完整 (5个)"
  else
    echo "  ⚠️ 规则系统不完整 ($rules_count/5个)"
  fi
  
  # 检查命令系统
  local commands_count=$(ls -1 .claude/commands/dd/*.md 2>/dev/null | wc -l)
  echo "  📋 命令系统: $commands_count 个命令"
  
  # 检查脚本系统
  local scripts_count=$(ls -1 .claude/scripts/dd/*.sh 2>/dev/null | wc -l)
  echo "  🔧 脚本系统: $scripts_count 个脚本"
  
  echo ""
}

check_project_status() {
  echo "🎯 当前项目状态: "
  
  # 检查项目是否已初始化
  if [ -f ".claude/context/project.md" ]; then
    local project_name=$(grep "^name:" .claude/context/project.md 2>/dev/null | cut -d: -f2- | xargs)
    local project_type=$(grep "^type:" .claude/context/project.md 2>/dev/null | cut -d: -f2- | xargs)
    echo "  项目名称: ${project_name:-未设置}"
    echo "  项目类型: ${project_type:-未设置}"
    echo "  初始化状态: ✅ 已初始化"
  else
    echo "  初始化状态: ❌ 未初始化"
    echo "  💡 执行 /dd:init 开始初始化"
    echo ""
    return
  fi
  
  # 统计功能数量
  if [ -d ".claude/features" ]; then
    local features_count=$(ls -1d .claude/features/*/ 2>/dev/null | wc -l)
    echo "  功能数量: $features_count 个"
    
    # 统计活跃议题
    local active_issues=0
    for feature_dir in .claude/features/*/; do
      if [ -d "$feature_dir/issues" ]; then
        local issue_count=$(find "$feature_dir/issues" -name "*.md" -exec grep -l "status: 进行中" {} \; 2>/dev/null | wc -l)
        active_issues=$((active_issues + issue_count))
      fi
    done
    echo "  活跃议题: $active_issues 个"
  else
    echo "  功能数量: 0 个"
    echo "  活跃议题: 0 个"
  fi
  
  echo ""
}

system_health_check() {
  echo "🏥 系统健康度检查: "
  
  local health_score=0
  local total_checks=4
  
  # 检查必要文件
  if [ -f ".claude/CLAUDE.md" ]; then
    echo "  ✅ CLAUDE.md 配置文件"
    health_score=$((health_score + 1))
  else
    echo "  ❌ CLAUDE.md 配置文件缺失"
  fi
  
  if [ -f ".claude/rules/absolute.md" ]; then
    echo "  ✅ 绝对规则文件"
    health_score=$((health_score + 1))
  else
    echo "  ❌ 绝对规则文件缺失"
  fi
  
  if [ -d ".claude/context" ]; then
    echo "  ✅ 上下文目录结构"
    health_score=$((health_score + 1))
  else
    echo "  ❌ 上下文目录结构缺失"
  fi
  
  # 检查命令脚本完整性
  local required_commands=19
  local actual_commands=$(ls -1 .claude/commands/dd/*.md 2>/dev/null | wc -l)
  if [ "$actual_commands" -eq "$required_commands" ]; then
    echo "  ✅ 命令系统完整"
    health_score=$((health_score + 1))
  else
    echo "  ⚠️ 命令系统: $actual_commands/$required_commands"
  fi
  
  # 计算健康度
  local health_percent=$((health_score * 100 / total_checks))
  echo ""
  if [ "$health_percent" -eq 100 ]; then
    echo "🟢 系统健康度: 优秀 (100%)"
  elif [ "$health_percent" -ge 75 ]; then
    echo "🟡 系统健康度: 良好 ($health_percent%)"
  else
    echo "🔴 系统健康度: 需要关注 ($health_percent%)"
  fi
  
  echo ""
}

show_usage_tips() {
  echo "💡 使用建议: "
  echo "  • 新项目: /dd:init → /dd:prd → /dd:framework-init"
  echo "  • 已有项目: /dd:init --analyze → /dd:framework-audit"
  echo "  • 智能咨询: /dd:chat"
  echo "  • 查看状态: /dd:status"
  echo "  • 获取帮助: /dd:help"
}

main() {
  show_version
  check_components
  check_project_status
  system_health_check
  show_usage_tips
}

main "$@"