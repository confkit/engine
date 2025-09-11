#!/bin/bash

# 获取当前 Claude Code 会话的对话历史
# 解析 ~/.claude/projects/ 目录下的会话文件

set -e

# 获取当前项目的会话目录
PROJECT_PATH=$(pwd | sed 's|/|-|g')
SESSION_DIR="$HOME/.claude/projects/$PROJECT_PATH"

if [ ! -d "$SESSION_DIR" ]; then
    echo "错误: 未找到当前项目的会话目录"
    echo "预期路径: $SESSION_DIR"
    exit 1
fi

# 找到真正的当前活跃会话（按内部时间戳）
echo "🔍 正在分析会话文件..."
CURRENT_SESSION=""
LATEST_TIMESTAMP=""

for session_file in "$SESSION_DIR"/*.jsonl; do
    if [ -f "$session_file" ]; then
        # 获取每个会话文件的最后时间戳
        last_timestamp=$(tail -1 "$session_file" 2>/dev/null | jq -r '.timestamp // empty' 2>/dev/null)
        if [ -n "$last_timestamp" ] && [ "$last_timestamp" != "null" ]; then
            # 转换为可比较的格式并比较
            if [ -z "$LATEST_TIMESTAMP" ] || [ "$last_timestamp" \> "$LATEST_TIMESTAMP" ]; then
                LATEST_TIMESTAMP="$last_timestamp"
                CURRENT_SESSION="$session_file"
            fi
        fi
    fi
done

if [ -z "$CURRENT_SESSION" ]; then
    echo "错误: 未找到活跃的会话文件"
    exit 1
fi

LATEST_SESSION="$CURRENT_SESSION"

echo ""
echo "=== 当前会话信息 ==="
echo "会话文件: $(basename "$LATEST_SESSION")"
echo "会话ID: $(basename "$LATEST_SESSION" .jsonl)"
echo "文件修改时间: $(stat -f "%Sm" -t "%Y-%m-%d %H:%M:%S" "$LATEST_SESSION")"
echo "内部最后时间戳: $LATEST_TIMESTAMP"

# 验证是否是真正的当前会话（简化检查）
current_date=$(date +"%Y-%m-%d")
session_date=$(echo "$LATEST_TIMESTAMP" | cut -d'T' -f1)

if [ "$current_date" = "$session_date" ]; then
    echo "✅ 确认这是当前活跃会话（今日会话）"
else
    echo "⚠️  警告: 此会话日期为 $session_date，可能不是当前会话"
fi
echo ""

# 提取对话历史（简化版本，提取用户和助手消息）
echo "=== 对话历史 ==="
echo ""

# 使用 jq 解析 JSONL 文件，提取对话摘要
if command -v jq >/dev/null 2>&1; then
    echo "📊 对话统计信息:"
    user_count=$(cat "$LATEST_SESSION" | jq -r 'select(.type == "user")' | wc -l | tr -d ' ')
    assistant_count=$(cat "$LATEST_SESSION" | jq -r 'select(.type == "assistant")' | wc -l | tr -d ' ')
    echo "  - 用户消息: $user_count 条"
    echo "  - AI回复: $assistant_count 条"
    echo ""
    
    echo "🎯 对话主要内容:"
    # 提取前5轮用户-AI对话
    cat "$LATEST_SESSION" | \
    jq -r --slurp '
        [.[] | select(.type == "user" or .type == "assistant")] | 
        sort_by(.timestamp) |
        .[:10] |
        .[] |
        if .type == "user" then 
            "用户: " + (.message.content // "")
        elif .type == "assistant" then 
            "AI: " + ((.message.content[0].text // .message.content // "") | 
            if length > 200 then .[:200] + "..." else . end)
        else empty end
    ' 2>/dev/null
    
    echo ""
    echo "📝 建议的对话记录方式:"
    echo "1. 手动总结关键对话要点"
    echo "2. 或使用简化格式: '用户提出了XX需求，AI分析并实现了XX功能'"
    echo "3. 重点记录决策过程和最终结论"
else
    echo "需要安装 jq 来解析会话文件: brew install jq"
    echo ""
    echo "或者查看原始会话文件:"
    echo "cat \"$LATEST_SESSION\""
fi

echo ""
echo "=== 对话记录建议 ==="
echo ""
echo "❌ 问题: 完整对话记录过于冗长，不适合直接使用"
echo ""
echo "✅ 推荐方案:"
echo "1. **简化记录**: 总结关键决策和结论，而不是逐句记录"
echo "2. **重点描述**: '用户要求优化XX，AI分析问题并实现了YY改进'"
echo "3. **决策要点**: 记录重要的技术选择和理由"
echo ""
echo "💡 示例格式:"
echo '  "conversation": "用户反馈init.md层次不清晰，AI分析后重构了文档结构，简化了4-5级标题为2-3级，优化了执行流程描述，提升了可读性"'
echo ""
echo "🎯 总结: 用统计信息和关键要点代替完整对话记录"