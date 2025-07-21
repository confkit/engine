//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Global variables

use crate::types::config::ConfKitConfig;
use crate::types::config::Engine;

// 缓存全局配置文件
pub static mut CONFIG: Option<ConfKitConfig> = None;

// 缓存当前宿主机使用的引擎
pub static mut ENGINE: Option<Engine> = None;
