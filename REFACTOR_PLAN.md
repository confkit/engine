# Runner 重构计划

## 问题分析

当前 `runner.rs` 承担了过多业务逻辑，应该按照职责分离原则进行重构：
- **Runner**: 应该只负责任务的编排和调度
- **Task**: 应该负责具体的业务逻辑执行和状态管理

## 重构目标

将 Runner 从业务执行者转变为任务协调者，Task 从数据载体转变为业务执行者。

## 详细重构计划

### 1. Task 职责扩展

**当前 Task 职责**：
- 任务ID生成
- 日志记录
- 时间管理

**重构后 Task 职责**：
- 任务生命周期管理（准备、执行、清理）
- 业务逻辑执行
- 步骤执行编排
- 执行结果收集和分析
- 任务信息打印和摘要输出

### 2. Runner 职责简化

**当前 Runner 职责**：
- 项目配置加载
- 执行上下文创建
- 任务创建
- 步骤执行编排
- 结果收集
- 信息打印
- 清理工作

**重构后 Runner 职责**：
- 任务初始化（配置加载、上下文创建）
- 任务启动
- 高层异常处理
- 资源管理

### 3. 具体重构内容

#### 3.1 移动到 Task 的方法
从 `runner.rs` 移动以下方法到 `task.rs`：

- `prepare_task()` → `Task::prepare()`
- `post_task()` → `Task::cleanup()`  
- `execute_all_steps()` → `Task::execute_steps()`
- `print_task_info()` → `Task::print_info()`
- `print_execution_summary()` → `Task::print_summary()`

#### 3.2 Task 新增字段
```rust
pub struct Task {
    // 现有字段
    pub id: String,
    pub started_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
    pub log_path: String,
    
    // 新增字段
    pub context: ExecutionContext,
    pub project_config: ConfKitProjectConfig,
    pub results: Vec<StepResult>,
}
```

#### 3.3 Task 新增方法
```rust
impl Task {
    // 任务生命周期管理
    pub async fn prepare(&self) -> Result<()>
    pub async fn execute(&mut self) -> Result<()>
    pub async fn cleanup(&self) -> Result<()>
    
    // 业务逻辑执行
    pub async fn execute_steps(&mut self) -> Result<Vec<StepResult>>
    
    // 信息输出
    pub fn print_info(&self) -> Result<()>
    pub fn print_summary(&self) -> Result<()>
    
    // 执行分析
    pub fn get_execution_stats(&self) -> (usize, usize, usize) // success, failed, skipped
    pub fn get_total_duration(&self) -> u64
}
```

#### 3.4 Runner 简化后的结构
```rust
impl Runner {
    pub async fn new(
        space_name: &str, 
        project_name: &str, 
        environment_from_args: HashMap<String, String>
    ) -> Result<Self>
    
    pub async fn start(&mut self) -> Result<()> {
        // 只负责任务的启动和异常处理
        self.task.prepare().await?;
        self.task.execute().await?;
        self.task.cleanup().await?;
        self.task.finish();
        Ok(())
    }
}
```

### 4. 重构步骤

1. **阶段一：Task 扩展**
   - 为 Task 添加新字段
   - 实现任务准备方法
   - 实现信息打印方法

2. **阶段二：业务逻辑迁移**  
   - 迁移步骤执行逻辑
   - 迁移结果收集逻辑
   - 迁移摘要输出逻辑

3. **阶段三：清理和优化**
   - 迁移清理逻辑
   - 简化 Runner 实现
   - 调整错误处理

4. **阶段四：测试验证**
   - 单元测试更新
   - 集成测试验证
   - 性能测试确认

### 5. 重构后的调用流程

```
Runner::new() -> 加载配置，创建 Task
Runner::start() -> Task::prepare() -> Task::execute() -> Task::cleanup()
```

### 6. 重构收益

- **职责清晰**：Runner 专注调度，Task 专注执行
- **可测试性**：Task 的业务逻辑更容易单独测试
- **可维护性**：业务逻辑集中在 Task 中，便于维护
- **可扩展性**：Task 可以独立扩展新的业务功能

## 注意事项

1. 保持 API 兼容性，不影响外部调用
2. 确保错误处理逻辑的完整性
3. 维持日志输出的一致性
4. 考虑并发安全（如果有并发需求）

## 时间估算

- 阶段一：1-2 天
- 阶段二：2-3 天  
- 阶段三：1 天
- 阶段四：1-2 天

**总计：5-8 天**
