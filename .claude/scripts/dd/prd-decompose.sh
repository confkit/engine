#!/bin/bash

# 需求拆解脚本
# 基于PRD和架构设计拆解大功能模块, 规划开发路径

set -e

show_help() {
  echo "🎯 DD 需求拆解工具"
  echo "=================="
  echo ""
  echo "功能: 基于PRD和架构设计, 将项目需求拆解为大功能模块"
  echo ""
  echo "用法: "
  echo "  $0                          # 自动分析并拆解需求"
  echo "  $0 --interactive            # 交互式拆解过程"
  echo "  $0 --show                   # 显示现有拆解结果"
  echo ""
  echo "执行条件: "
  echo "  • 项目已完成初始化 (dd:init)"  # 支持 --analyze 参数
  echo "  • 已完成需求设计 (dd:prd)"  
  echo "  • 已完成架构设计 (dd:framework-init)"
  echo ""
  echo "输出: "
  echo "  • 基于现有 PRD 进行功能识别和拆解"
  echo "  • 与用户确认功能列表后批量创建功能"
  echo "  • 提供下一步操作建议"
}

check_prerequisites() {
  local missing=false
  
  echo "🔍 检查前置条件..."
  
  # 检查项目是否已初始化
  if [ ! -f ".claude/context/project.md" ]; then
    echo "❌ 项目未初始化, 请先执行 /dd:init (支持 --analyze 参数)"
    missing=true
  fi
  
  # 检查是否已完成需求设计
  if [ ! -f ".claude/context/project.md" ] || ! grep -q "prd_completed" .claude/context/project.md 2>/dev/null; then
    echo "⚠️  建议先完成需求设计 (/dd:prd)"
  fi
  
  # 检查是否已完成架构设计
  if [ ! -f ".claude/context/architecture.md" ]; then
    echo "❌ 架构设计未完成, 请先执行 /dd:framework-init"
    missing=true
  fi
  
  if [ "$missing" = true ]; then
    echo ""
    echo "🚨 前置条件不满足, 无法执行需求拆解"
    exit 1
  fi
  
  echo "✅ 前置条件检查通过"
}

load_project_context() {
  echo "📋 加载项目上下文..."
  
  # 读取项目信息
  if [ -f ".claude/context/project.md" ]; then
    PROJECT_NAME=$(grep "^name:" .claude/context/project.md | cut -d: -f2- | xargs)
    PROJECT_TYPE=$(grep "^type:" .claude/context/project.md | cut -d: -f2- | xargs)
    echo "  项目名称: $PROJECT_NAME"
    echo "  项目类型: $PROJECT_TYPE"
  fi
  
  # 读取架构信息
  if [ -f ".claude/context/architecture.md" ]; then
    echo "  ✅ 架构设计文档已加载"
  fi
  
  # 读取技术栈信息
  if [ -f ".claude/context/tech-stack.md" ]; then
    echo "  ✅ 技术栈信息已加载"
  fi
}

show_existing_features() {
  echo "📋 当前项目功能状态: "
  echo "===================="
  
  if [ -d ".claude/features" ] && [ "$(ls -A .claude/features 2>/dev/null)" ]; then
    echo "✅ 已创建的功能模块:"
    for feature_dir in .claude/features/*/; do
      if [ -d "$feature_dir" ]; then
        feature_name=$(basename "$feature_dir")
        echo "  - $feature_name"
      fi
    done
  else
    echo "❌ 尚未创建功能模块"
    echo "💡 请执行 /dd:prd-decompose 开始功能拆解"
  fi
}

perform_decomposition() {
  echo "🧠 开始基于现有 PRD 进行功能拆解分析..."
  echo "📋 读取项目需求文档进行智能分析..."
  echo ""
  
  local project_name=${PROJECT_NAME:-"未知项目"}
  
  echo "📄 分析 $project_name 的需求文档"
  echo "🤖 使用深度思考智能体进行功能模块识别..."
  echo ""
  echo "💡 智能体将基于以下信息进行分析:"
  echo "   • 现有 PRD 需求文档"
  echo "   • 架构设计和技术选型"  
  echo "   • 功能模块拆解策略"
  echo "   • 依赖关系和优先级规划"
  echo ""
  echo "⏳ 等待 AI 完成分析并提供功能列表供用户确认..."
}

show_completion_message() {
  echo ""
  echo "🎯 需求拆解分析完成！"
  echo "💬 AI 已完成功能模块识别，等待用户确认"
  echo ""
  echo "📝 确认功能列表后将自动执行: "
  echo "   /dd:feature-add <功能1>"
  echo "   /dd:feature-add <功能2>"
  echo "   /dd:feature-add <功能3>"
  echo "   ..."
  echo ""
  echo "💡 查看已创建功能状态: "
  echo "   /dd:feature-status"
}

main() {
  case "${1:-}" in
    "--help"|"-h"|"help")
      show_help
      ;;
    "--show"|"show")
      show_existing_features
      ;;
    "--interactive"|"-i")
      echo "🎯 交互式需求拆解"
      echo "=================="
      check_prerequisites
      load_project_context
      echo ""
      echo "💬 提示: 此功能将启动深度对话进行需求分析"
      echo "🤖 请使用 /dd:chat 进行交互式需求拆解讨论"
      ;;
    "")
      echo "🎯 DD 需求拆解 - 自动分析模式"
      echo "=============================="
      check_prerequisites
      load_project_context
      perform_decomposition
      
      show_completion_message
      ;;
    *)
      echo "❌ 未知参数: $1"
      echo "💡 使用 $0 --help 查看帮助"
      exit 1
      ;;
  esac
}

main "$@"