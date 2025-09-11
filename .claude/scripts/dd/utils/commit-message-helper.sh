#!/bin/bash

# Git Commit Message 智能生成助手
# 基于代码变更自动生成规范的 commit message 建议

# 获取提交类型说明
get_commit_type_desc() {
  case "$1" in
    "feat") echo "新功能" ;;
    "fix") echo "错误修复" ;;
    "docs") echo "文档更新" ;;
    "style") echo "代码格式（不影响功能的变更）" ;;
    "refactor") echo "代码重构（既不修复错误也不添加功能）" ;;
    "perf") echo "性能优化" ;;
    "build") echo "构建系统或外部依赖变更" ;;
    "ci") echo "CI配置文件和脚本变更" ;;
    "chore") echo "其他变更" ;;
    "revert") echo "撤销之前的提交" ;;
    *) echo "未知类型" ;;
  esac
}

# 分析文件变更类型
analyze_changes() {
  local staged_files=$(git diff --cached --name-only 2>/dev/null)
  local modified_files=$(git diff --name-only 2>/dev/null)
  local all_files="$staged_files $modified_files"
  
  if [ -z "$all_files" ]; then
    echo "没有检测到文件变更"
    return 1
  fi
  
  echo "检测到的文件变更:"
  echo "$all_files" | tr ' ' '\n' | sort -u | head -10
  
  # 分析变更类型
  local has_new_files=false
  local has_docs=false
  local has_config=false
  local has_src_files=false
  
  for file in $all_files; do
    if [ ! -f "$file" ] && git ls-files --error-unmatch "$file" >/dev/null 2>&1; then
      # 新增文件
      has_new_files=true
    fi
    
    case "$file" in
      *.md|*.txt|docs/*|README*)
        has_docs=true
        ;;
      package.json|*.config.*|.*rc|.*ignore|Dockerfile|docker-compose.yml)
        has_config=true
        ;;
      src/*|lib/*|*.js|*.ts|*.jsx|*.tsx|*.py|*.go|*.java|*.c|*.cpp)
        has_src_files=true
        ;;
    esac
  done
  
  echo ""
  echo "变更分析:"
  echo "  新增文件: $has_new_files"
  echo "  文档文件: $has_docs"
  echo "  配置文件: $has_config"
  echo "  源代码文件: $has_src_files"
  
  # 推荐commit类型
  local recommended_types=()
  
  if [ "$has_docs" = true ]; then
    recommended_types+=("docs")
  fi
  
  if [ "$has_config" = true ]; then
    recommended_types+=("build" "ci" "chore")
  fi
  
  if [ "$has_src_files" = true ]; then
    if [ "$has_new_files" = true ]; then
      recommended_types+=("feat")
    else
      recommended_types+=("feat" "fix" "refactor" "perf")
    fi
  fi
  
  if [ ${#recommended_types[@]} -eq 0 ]; then
    recommended_types+=("chore")
  fi
  
  echo ""
  echo "推荐的commit类型:"
  for type in "${recommended_types[@]}"; do
    echo "  $type: $(get_commit_type_desc "$type")"
  done
}

# 生成commit message建议
generate_commit_message_suggestions() {
  echo ""
  echo "🤖 AI 生成的 Commit Message 建议:"
  echo "=================================="
  
  # 获取变更的文件列表和统计
  local staged_files=$(git diff --cached --name-only 2>/dev/null)
  local modified_files=$(git diff --name-only 2>/dev/null) 
  local all_files="$staged_files $modified_files"
  
  if [ -z "$all_files" ]; then
    echo "⚠️ 没有检测到文件变更，无法生成建议"
    return 1
  fi
  
  # 统计变更
  local file_count=$(echo "$all_files" | tr ' ' '\n' | sort -u | wc -l | tr -d ' ')
  local added_lines=$(git diff --cached --numstat 2>/dev/null | awk '{sum+=$1} END {print sum+0}')
  local deleted_lines=$(git diff --cached --numstat 2>/dev/null | awk '{sum+=$2} END {print sum+0}')
  
  # 基于文件类型和变更规模生成建议
  echo ""
  echo "📊 变更统计:"
  echo "  文件数量: $file_count 个"
  echo "  新增行数: $added_lines 行"
  echo "  删除行数: $deleted_lines 行"
  echo ""
  
  # 生成多个commit message建议
  echo "💡 建议的完整 Commit Message:"
  echo ""
  
  local suggestions=()
  
  # 检查主要变更类型并生成对应建议
  if echo "$all_files" | grep -q "\.md\|README\|docs/"; then
    suggestions+=("docs: 更新项目文档和说明")
    suggestions+=("docs: 完善文档内容，增加使用示例")
  fi
  
  
  if echo "$all_files" | grep -q "config\|package\.json\|\..*rc\|\.claude"; then
    suggestions+=("build: 更新构建配置和依赖管理")
    suggestions+=("chore: 调整项目配置，优化开发环境")
  fi
  
  # 检查脚本和工具文件
  if echo "$all_files" | grep -q "\.sh\|scripts/\|utils/"; then
    suggestions+=("feat: 新增工具脚本，提升开发效率")
    suggestions+=("fix: 修复脚本问题，改善工具体验")
    suggestions+=("chore: 优化开发工具，完善工作流")
  fi
  
  # 检查是否有git相关修改
  if echo "$all_files" | grep -q "git\|commit"; then
    suggestions+=("fix: 修复git操作提示，明确用户手动执行要求")
    suggestions+=("chore: 规范git工作流，强化安全边界")
  fi
  
  # 通用建议（基于变更规模）
  if [ "$file_count" -gt 5 ]; then
    suggestions+=("refactor: 重构代码结构，提高可维护性")
    suggestions+=("feat: 实现新功能模块，扩展系统能力")
  elif [ "$added_lines" -gt 50 ]; then
    suggestions+=("feat: 新增功能实现，完善业务逻辑")
    suggestions+=("fix: 修复关键问题，提升系统稳定性")
  else
    suggestions+=("fix: 修复小问题，优化用户体验")
    suggestions+=("style: 代码格式调整，统一编码风格")
  fi
  
  # 输出建议（去重并限制数量）
  printf '%s\n' "${suggestions[@]}" | sort -u | head -8 | nl -s '. '
  
  echo ""
  echo "🎯 使用示例:"
  echo "  git add <files>"
  echo "  git commit -m \"feat: 实现用户认证功能，支持多种登录方式\""
  echo "  # 或选择上述任一建议，根据实际变更内容调整"
  echo ""
  echo "📝 Commit Message 格式说明:"
  echo "  <type>: <description>"
  echo "  • type: 变更类型（feat/fix/docs/style等）"
  echo "  • description: 简洁描述变更内容（建议50字以内）"
}

# 主函数
main() {
  local command="$1"
  
  case "$command" in
    "analyze")
      analyze_changes
      ;;
    "suggest")
      generate_commit_message_suggestions
      ;;
    "help"|*)
      echo "Git Commit Message 智能助手"
      echo ""
      echo "用法: $0 {analyze|suggest|help}"
      echo ""
      echo "命令说明:"
      echo "  analyze  - 分析当前文件变更类型"
      echo "  suggest  - 生成 commit message 建议"
      echo "  help     - 显示帮助信息"
      echo ""
      echo "约定式提交类型:"
      local types="feat fix docs style refactor perf build ci chore revert"
      for type in $types; do
        echo "  $type: $(get_commit_type_desc "$type")"
      done
      ;;
  esac
}

# 如果脚本被直接调用
if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
  main "$@"
fi