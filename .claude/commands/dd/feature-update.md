---
allowed-tools: Task, Read, Write, Edit, MultiEdit, Bash
---

# Feature Update

智能更新功能整体状态和进度，自动遍历并更新所有子议题状态，提供功能级别的进度管理。

## Usage

### 自动更新（推荐）

```bash
/dd:feature-update <feature_name>
```

### 手动指定状态

```bash
/dd:feature-update <feature_name> --status "已完成"
```

## Instructions

### 1. 自动更新执行流程

AI 会自动：

1. 批量更新所有子议题状态
2. 聚合计算功能整体进度
3. 更新功能文档
4. 生成进度报告

### 2. 功能状态读取

自动调用状态脚本获取功能和所有子议题信息，所有用法选项：

```bash
bash .claude/scripts/dd/query/get-feature.sh "<feature_name>"                    # 默认读取 overview.md
bash .claude/scripts/dd/query/get-feature.sh --status-only "<feature_name>"     # 仅显示状态信息，不显示文档内容
bash .claude/scripts/dd/query/get-feature.sh --all "<feature_name>"             # 读取所有文档 (overview + technical + acceptance) - 推荐用于更新操作
bash .claude/scripts/dd/query/get-feature.sh --overview "<feature_name>"        # 仅读取功能概述文档 (overview.md)
bash .claude/scripts/dd/query/get-feature.sh --technical "<feature_name>"       # 仅读取技术方案文档 (technical.md)
bash .claude/scripts/dd/query/get-feature.sh --acceptance "<feature_name>"      # 仅读取验收标准文档 (acceptance.md)
```

### 3. 子议题批量更新

在更新功能状态前，自动遍历并更新所有子议题：

- 逐个分析每个议题的实际状态
- 智能推断议题进度和状态变更
- 更新所有议题文档的元数据

### 4. 功能整体状态计算

基于所有子议题状态计算功能整体状态：

- **未开始** - 所有议题都未开始
- **进行中** - 部分议题已开始或完成
- **已完成** - 所有议题都已完成

### 5. 进度自动聚合

智能计算功能整体进度：

- 基于子议题完成度加权平均
- 考虑议题优先级和复杂度
- 更新功能文档的 progress 字段

### 6. 功能信息收集

```bash
# 读取功能状态和所有子议题信息 - 所有用法选项：
bash .claude/scripts/dd/query/get-feature.sh "<feature_name>"                    # 默认读取 overview.md
bash .claude/scripts/dd/query/get-feature.sh --status-only "<feature_name>"     # 仅显示状态信息，不显示文档内容
bash .claude/scripts/dd/query/get-feature.sh --all "<feature_name>"             # 读取所有文档 (overview + technical + acceptance) - 推荐用于更新分析
bash .claude/scripts/dd/query/get-feature.sh --overview "<feature_name>"        # 仅读取功能概述文档 (overview.md)
bash .claude/scripts/dd/query/get-feature.sh --technical "<feature_name>"       # 仅读取技术方案文档 (technical.md)
bash .claude/scripts/dd/query/get-feature.sh --acceptance "<feature_name>"      # 仅读取验收标准文档 (acceptance.md)
```

### 7. 子议题批量分析和更新

对每个子议题执行：

1. 读取议题当前状态
2. 分析代码变更和 TODO 完成度
3. 智能推断议题新状态
4. 更新议题文档
5. 记录更新日志

### 8. 功能状态聚合

基于更新后的子议题状态：

- 计算功能整体进度百分比
- 确定功能整体状态
- 识别阻塞问题和风险

### 9. 功能文档更新

文档位置:
`.claude/features/<feature_name>/overview.md`

更新功能主文档 `overview.md` 的关键信息：

- `status` - 功能整体状态
- `progress` - 整体完成进度
- `issues_total` - 总议题数量
- `issues_completed` - 已完成议题数

### 10. 更新项目状态上下文

功能文档更新完成后, 智能分析并更新项目整体状态:

```bash
# 1. 获取所有功能列表
feature_list=$(find .claude/features -maxdepth 1 -type d -not -path ".claude/features" | xargs -I {} basename {})

# 2. 收集所有功能状态信息
echo "=== 项目功能状态分析 ==="
for feature in $feature_list; do
    echo "功能: $feature"
    bash .claude/scripts/dd/query/get-feature.sh --status-only "$feature"
    echo ""
done

# 3. AI 根据收集到的所有功能状态，智能分析并更新 .claude/context/current-status.md
```

### 11. 状态计算逻辑

- **未开始**: 所有议题未开始
- **进行中**: 部分议题已开始或完成
- **已完成**: 所有议题已完成

进度 = 已完成议题数 / 总议题数 × 100%

## Important Notes

提供功能级别的进度管理，自动聚合子议题状态，确保功能状态实时准确。
