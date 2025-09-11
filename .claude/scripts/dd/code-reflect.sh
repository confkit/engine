#!/bin/bash

# 代码变更反思脚本
# 分析 Git 未提交的文件变动

echo "🔍 代码变更反思分析"
echo "===================="
echo ""

## 1. 详细 Git 状态检查

if [ -x ".claude/scripts/dd/utils/git-check.sh" ]; then
    echo "🔍 执行详细Git状态检查..."
    bash .claude/scripts/dd/utils/git-check.sh full-check
    echo ""
fi

## 2. Git 变更检测

echo "🔍 检测代码变更..."

# 检查是否在 git 仓库中
if ! git rev-parse --git-dir > /dev/null 2>&1; then
  echo "❌ 当前目录不是 Git 仓库"
  echo "💡 请在 Git 仓库根目录下运行此命令"
  exit 1
fi

# 检查工作区和暂存区变更
working_changes=$(git diff --name-only 2>/dev/null | wc -l)
staged_changes=$(git diff --staged --name-only 2>/dev/null | wc -l)

if [ "$working_changes" -eq 0 ] && [ "$staged_changes" -eq 0 ]; then
  echo "✅ 没有检测到未提交的代码变更"
  echo ""
  echo "💡 如需反思已提交的变更, 请指定提交范围: "
  echo "   git show <commit-hash>"
  echo "   git diff <commit-hash>..HEAD"
  echo ""
  echo "🎯 其他建议: "
  echo "  • 使用 /dd:chat 进行技术问题讨论"
  echo "  • 使用 /dd:status 查看项目整体状态"
  echo "  • 使用 /dd:feature-start <功能> 开始新功能开发"
  exit 0
fi

echo "📊 变更概览: "
echo "  工作区变更: $working_changes 个文件"
echo "  暂存区变更: $staged_changes 个文件"
echo ""

## 2. 变更文件分析

echo "📝 变更文件列表: "
echo ""

# 显示工作区变更文件
if [ "$working_changes" -gt 0 ]; then
  echo "🔄 工作区变更: "
  git diff --name-status 2>/dev/null | while read -r status file; do
    case $status in
      A) echo "  ✅ 新增: $file" ;;
      M) echo "  📝 修改: $file" ;;
      D) echo "  ❌ 删除: $file" ;;
      R*) echo "  🔄 重命名: $file" ;;
      *) echo "  🔸 $status: $file" ;;
    esac
  done
fi

# 显示暂存区变更文件
if [ "$staged_changes" -gt 0 ]; then
  echo ""
  echo "📦 暂存区变更: "
  git diff --staged --name-status 2>/dev/null | while read -r status file; do
    case $status in
      A) echo "  ✅ 新增: $file" ;;
      M) echo "  📝 修改: $file" ;;
      D) echo "  ❌ 删除: $file" ;;
      R*) echo "  🔄 重命名: $file" ;;
      *) echo "  🔸 $status: $file" ;;
    esac
  done
fi

echo ""

## 3. 代码统计分析

echo "📈 代码变更统计: "
git diff --stat HEAD 2>/dev/null | tail -1
echo ""

## 4. 查找关联的任务

echo "🎯 任务关联分析: "
current_issues=$(find .claude/features -name "*.md" -path "*/issues/*" -exec grep -l "^status: 进行中" {} \; 2>/dev/null)

if [ -n "$current_issues" ]; then
  echo "  发现进行中的任务: "
  echo "$current_issues" | while read -r issue_file; do
    issue_name=$(grep "^name:" "$issue_file" | sed 's/^name: *//')
    feature_name=$(echo "$issue_file" | sed 's|.*/features/\([^/]*\)/.*|\1|')
    issue_num=$(basename "$issue_file" .md)
    echo "    📋 $feature_name:$issue_num - $issue_name"
  done
else
  echo "  ℹ️  未发现进行中的任务"
fi
echo ""

## 5. 最近提交历史

echo "📚 最近开发记录: "
git log --oneline -5 2>/dev/null | while read -r line; do
  echo "  🔸 $line"
done
echo ""

## 6. 提供反思建议

echo "🤔 代码反思建议: "
echo ""
echo "基于检测到的变更, 建议进行以下检查: "
echo ""

# 根据变更文件类型提供建议
has_js_ts=$(git diff --name-only HEAD 2>/dev/null | grep -E '\.(js|ts|jsx|tsx)$' | wc -l)
has_css=$(git diff --name-only HEAD 2>/dev/null | grep -E '\.(css|scss|sass|less)$' | wc -l)
has_config=$(git diff --name-only HEAD 2>/dev/null | grep -E '\.(json|yaml|yml|toml|ini)$' | wc -l)
has_docs=$(git diff --name-only HEAD 2>/dev/null | grep -E '\.(md|txt|rst)$' | wc -l)

echo "🔍 建议检查项目: "

if [ "$has_js_ts" -gt 0 ]; then
  echo "  • JavaScript/TypeScript 代码质量"
  echo "    - 代码风格和格式是否一致"
  echo "    - 是否有适当的错误处理"
  echo "    - 函数和变量命名是否清晰"
  echo "    - 是否遵循项目编码规范"
fi

if [ "$has_css" -gt 0 ]; then
  echo "  • 样式代码检查"
  echo "    - CSS类名和样式组织"
  echo "    - 响应式设计考虑"
  echo "    - 浏览器兼容性"
fi

if [ "$has_config" -gt 0 ]; then
  echo "  • 配置文件变更"
  echo "    - 配置项的合理性"
  echo "    - 敏感信息是否正确处理"
  echo "    - 环境配置的一致性"
fi

if [ "$has_docs" -gt 0 ]; then
  echo "  • 文档更新检查"
  echo "    - 文档内容是否准确"
  echo "    - 是否与代码变更保持同步"
fi

echo ""
echo "⭐ 通用质量检查: "
echo "  • 代码是否符合项目架构设计"
echo "  • 变更是否引入新的依赖或风险"
echo "  • 是否需要更新相关测试用例"
echo "  • 性能影响是否在可接受范围内"
echo "  • 安全性考虑是否充分"
echo ""

## 7. 交互式选项

echo "🎯 下一步建议: "
echo ""
echo "1. 📖 查看具体变更内容: "
echo "   git diff              # 查看工作区变更"
echo "   git diff --staged     # 查看暂存区变更"
echo ""
echo "2. 🔧 代码质量检查: "
echo "   npm run lint         # 或其他代码检查工具"
echo ""
echo "3. 💬 深度技术讨论: "
echo "   /dd:chat             # 使用DD智能对话"
echo "   [描述您的变更和关注点, 获得深度分析建议]"
echo ""
echo "4. 📝 用户可手动提交 (AI 禁止执行 git 写操作): "
echo "   git add <files>      # 用户手动添加要提交的文件"
echo "   🤖 获取 AI 生成的 commit message 建议:"
echo "   bash .claude/scripts/dd/utils/commit-message-helper.sh suggest"
echo "   git commit -m \"<使用AI建议的完整message>\" # 用户手动提交变更"
echo ""

## 8. 智能体分析提示

echo "🤖 智能分析建议: "
echo ""
echo "建议使用 /dd:chat 进行深度代码分析: "
echo ""
echo "示例对话: "
echo "  /dd:chat"
echo "  我刚刚修改了以下文件: [列出主要文件]"
echo "  主要变更包括: [描述主要变更]"
echo "  请帮我分析这些变更的合理性和潜在风险"
echo "  特别关注: [具体关注点]"
echo ""

## 9. 项目状态提醒

if [ -f ".claude/context/current-status.md" ]; then
  current_phase=$(grep "^project_phase:" .claude/context/current-status.md | sed 's/^project_phase: *//')
  echo "📊 当前项目阶段: $current_phase"
fi

echo ""
echo "✨ 代码反思分析完成"
echo ""
echo "💡 记住: 优秀的代码不仅要功能正确, 还要: "
echo "  • 可读性强, 易于维护"
echo "  • 遵循团队规范和最佳实践"
echo "  • 考虑安全性和性能"
echo "  • 有适当的测试覆盖"
echo "  • 符合系统整体架构"

exit 0