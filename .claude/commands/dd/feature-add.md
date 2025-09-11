---
allowed-tools: Task, Read, Write, Bash
---

# Feature Add

通过深度对话分析功能需求，生成完整功能文档结构。

## Usage

```bash
/dd:feature-add <功能名称>
```

## Instructions

### 1. 文档生成规范

**文档格式要求**

所有生成的文档内容必须遵循以下格式规范：

- **中英文混合文本**：英文单词与中文字符之间必须有一个空格
- **示例**：`这是一个 JWT 认证系统` 而不是 `这是一个JWT认证系统`
- **列表项格式**：多个列表项必须分行显示，每行以 "- " 开头
- **正确示例**：
  ```
  - 用户注册功能
  - 用户登录功能
  - 密码重置功能
  ```
- **错误示例**：`用户注册功能|用户登录功能|密码重置功能`
- **适用范围**：所有功能描述、技术方案、测试用例等文档

### 2. 深度需求对话

与用户进行深入讨论，明确：

- **功能目标**: 具体要实现什么
- **用户价值**: 为用户解决什么问题
- **核心功能点**: 主要功能模块
- **技术选型**: 使用的技术栈
- **复杂度评估**: 实现难度和工时
- **依赖关系**: 与其他功能的依赖

### 3. AI 质疑和优化

基于深度思考，主动质疑：

- 功能是否真正必要
- 实现方案是否合理
- 是否有更简单的替代方案
- 功能边界是否清晰

### 4. 确认功能规格

讨论确认后，整理出：

- 功能名称和目标描述
- 用户价值和使用场景
- 核心功能点列表
- 技术实现方案
- 测试策略和用例

### 5. 分步生成文档

基于深度对话结果，依次调用参数化脚本生成各部分文档：

**5.1 生成功能描述文档**

基于对话中获得的功能需求信息，构建 JSON 数据并调用脚本：

```bash
feature_data=$(cat << 'EOF'
{
  "goal": "从对话中提取的功能目标描述",
  "user_value": "从对话中明确的用户价值（markdown 列表格式，每行以 '- ' 开头）",
  "core_features": "核心功能点描述（markdown 列表格式，每行以 '- ' 开头）",
  "feature_boundary_include": "包含的功能范围（markdown 列表格式，每行以 '- ' 开头）",
  "feature_boundary_exclude": "明确排除的功能范围（markdown 列表格式，每行以 '- ' 开头）",
  "use_scenarios": "具体使用场景描述（markdown 列表格式，每行以 '- ' 开头）",
  "dependencies": "技术或功能依赖描述"
}
EOF
)

bash .claude/scripts/dd/generator/generate-feature-overview.sh "<功能名>" "$feature_data"
```

**5.2 生成技术方案文档**

基于对话中的技术选型和架构讨论，构建 JSON 数据并调用脚本：

```bash
technical_data=$(cat << 'EOF'
{
  "architecture_design": "从对话中确定的系统架构设计",
  "data_models": "数据模型和实体关系设计",
  "api_design": "API接口设计和规范",
  "database_design": "数据库设计方案",
  "security_considerations": "安全策略和考虑",
  "performance_requirements": "性能要求和优化策略",
  "tech_stack": "选择的技术栈组合",
  "external_integrations": "外部系统集成方案",
  "deployment_strategy": "部署和运维策略"
}
EOF
)

bash .claude/scripts/dd/generator/generate-feature-technical.sh "<功能名>" "$technical_data"
```

**5.3 生成验收标准文档**

基于对话中的功能需求和验收要求，构建 JSON 数据并调用脚本：

**重要验收原则**:

- **主要重点**: 功能点验收是核心，每个具体功能都要有明确的验收标准
- **按需补充**: 性能、安全性、其他要求仅在该功能确实需要时才添加
- **实际情况**: 根据功能的实际特点和复杂度确定验收维度，避免千篇一律

```bash
acceptance_data=$(cat << 'EOF'
{
  "functional_requirements": "核心功能点验收（markdown列表格式，详细列出每个功能点的验收标准）",
  "performance_requirements": "性能验收标准（仅当功能有性能要求时提供，markdown列表格式）",
  "security_requirements": "安全性验收标准（仅当功能涉及安全时提供，markdown列表格式）",
  "other_requirements": "其他验收要求（根据功能特点补充，如数据完整性、业务规则等，markdown列表格式）",
  "acceptance_criteria": "综合验收条件（整体功能完成的标准，markdown列表格式）"
}
EOF
)

bash .claude/scripts/dd/generator/generate-acceptance.sh "<功能名>" "$acceptance_data"
```

**5.4 完成功能创建和记录会话**

基于对话内容，手动总结关键要点并记录到会话历史：

```bash
# 查看当前会话统计信息
bash .claude/scripts/dd/utils/get-session.sh
```

### 6. 完成功能创建并智能引导下一步

AI 执行以下操作:

1. 验证所有功能文档生成完成
2. 初始化功能状态为"设计中"
3. **智能提出议题拆解方案**:
   - 基于功能复杂度和技术方案分析可能的议题结构
   - 向用户展示建议的拆解思路
   - 引导用户确认是否执行 `/dd:feature-decompose <功能名>`

4. **支持二次确认流程**:
   - 用户可提出调整意见
   - AI 根据反馈优化拆解建议
   - 多轮讨论直至用户满意后执行分解

**优势**: 无缝衔接功能创建到议题拆解的完整流程，确保开发路径清晰

### 7. 深度对话示例

**AI**: 您想添加什么功能？请描述一下核心需求.

**用户**: 我想添加一个用户认证系统.

**AI**: 让我深入了解一下这个功能:

1. **功能目标**: 您希望这个认证系统实现哪些具体功能？是否包括注册、登录、密码重置等？

2. **用户价值**: 这个系统主要为哪类用户服务？解决什么核心问题？

3. **技术考虑**: 基于您的项目架构, 倾向于使用什么认证方案？JWT、Session 还是 OAuth？

4. **质疑**: 是否考虑过使用第三方认证服务（如 Auth0）来简化实现？

_(继续深度对话，直到明确所有细节)_

### 8. AI 执行的关键原则

**JSON 数据格式要求**:

**重要**: JSON 中的列表类型数据必须使用正确的 markdown 格式:

```json
{
  "core_features": "- 用户注册功能\n- 用户登录功能\n- 密码重置功能",
  "use_scenarios": "- 新用户注册流程\n- 已有用户登录场景\n- 忘记密码重置场景"
}
```

**不要使用管道符分隔**: `"core_features": "用户注册功能|用户登录功能|密码重置功能"`

**参数化脚本的正确使用**

**重要: 必须传递完整的对话内容作为参数！**

1. **不要省略参数**: 每个重要信息都必须通过对应参数传递
2. **使用实际内容**: 不要使用占位符，使用从对话中获得的实际内容
3. **遵循格式要求**: 使用 markdown 列表格式 (每行以 `- ` 开头)
4. **验收重点原则**:
   - 功能点验收是核心重点，要详细具体
   - 性能、安全等其他验收根据功能实际情况补充
   - 避免为了完整性而添加不相关的验收项
5. **保证内容完整性**: 所有从用户对话中获得的信息都必须体现在最终生成的文档中

### 9. 执行示例

假设用户要求创建"用户认证系统", 经过对话得到以下信息后，AI必须构建正确的JSON格式:

```bash
# 第一步: 生成功能文档
feature_data=$(cat << 'EOF'
{
  "goal": "实现安全的用户登录认证",
  "user_value": "- 为用户提供安全便捷的身份验证服务\n- 保护用户账户和个人信息安全",
  "core_features": "- 用户注册功能\n- 用户登录功能\n- 密码重置功能\n- 邮箱验证功能",
  "feature_boundary_include": "- 基础的注册登录流程\n- 密码重置功能\n- 邮箱验证机制",
  "feature_boundary_exclude": "- 第三方登录集成\n- 多因素认证\n- 高级权限管理",
  "use_scenarios": "- 新用户注册流程\n- 已有用户登录场景\n- 忘记密码重置场景",
  "dependencies": "邮件服务、数据库系统"
}
EOF
)

bash .claude/scripts/dd/generator/generate-feature-overview.sh "用户认证系统" "$feature_data"

# 第二步: 生成技术文档
technical_data=$(cat << 'EOF'
{
  "tech_stack": "Node.js,React,JWT,MongoDB",
  "architecture_design": "前后端分离架构, JWT令牌认证",
  "data_models": "User表存储用户信息, 包含邮箱、密码哈希等字段",
  "api_design": "RESTful API设计, 包含注册、登录、重置密码接口",
  "security_considerations": "密码哈希存储, JWT令牌验证, 邮箱验证机制"
}
EOF
)

bash .claude/scripts/dd/generator/generate-feature-technical.sh "用户认证系统" "$technical_data"

# 第三步: 生成验收文档
# 注意：以下示例体现了验收原则 - 重点是功能点验收，其他根据实际需要补充
acceptance_data=$(cat << 'EOF'
{
  "functional_requirements": "- 用户可以成功注册新账户并收到确认邮件\n- 用户可以使用正确的邮箱和密码登录系统\n- 用户可以通过邮箱重置密码并设置新密码\n- 邮箱验证链接功能正常且有效期合理",
  "performance_requirements": "- 登录响应时间小于2秒\n- 注册流程完成时间小于30秒",
  "security_requirements": "- 密码使用bcrypt加密存储\n- JWT token生成和验证机制安全\n- 防止暴力破解攻击",
  "other_requirements": "- 输入验证和错误提示清晰准确\n- 邮件发送功能稳定可靠",
  "acceptance_criteria": "- 完整的注册登录流程可以正常工作\n- 用户数据安全得到保障\n- 系统响应性能满足用户体验要求"
}
EOF
)

bash .claude/scripts/dd/generator/generate-acceptance.sh "用户认证系统" "$acceptance_data"

# 第四步: 完成创建
# AI 直接执行功能创建完成逻辑:
# 1. 验证文档生成完整性
# 2. 初始化功能状态
# 3. 记录会话历史
# 4. 提供后续建议
```

### 10. 输出规范

确认所有细节后，会按顺序生成：

1. **overview.md** - 完整的功能需求文档（包含对话中的所有功能信息）
2. **technical.md** - 详细的技术实现方案（包含对话中的技术方案）
3. **acceptance.md** - 功能验收标准文档（包含验收要点和标准）
4. **完成提示** - 下一步操作建议

### 11. 质疑维度

- **必要性**: 是否真正解决核心痛点
- **复杂度**: 实现复杂度是否合理
- **替代方案**: 是否有更简单的解决方案
- **技术选型**: 技术方案是否适合项目
- **维护成本**: 长期维护复杂度考虑
- **扩展性**: 未来功能扩展的可能性

## Important Notes

通过深度对话分析需求，确保功能文档完整准确，为后续开发提供清晰指导。
