pub mod loader;
pub mod runner;
pub mod types;

// 重新导出主要的结构和功能
pub use loader::ProjectLoader;
pub use runner::{ProjectRunner, RunOptions};
pub use types::{
    ProjectConfig, ProjectRef, SpaceConfig, StepConfig, StepResult, TaskContext, TaskResult,
    TaskStatus,
};
