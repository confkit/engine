//! Author: xiaoYown
//! Created: 2025-09-13
//! Description: Interactive interface types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Interactive type for environment variable input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfKitInteractiveType {
    Input,
    Radio,
    Checkbox,
    Confirm,
}

/// Interactive environment variable configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEnvironmentInteractiveConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub interactive_type: ConfKitInteractiveType,
    pub prompt: String,
    pub default: Option<String>,
    #[serde(default = "default_required")]
    pub required: bool,
    pub options: Option<Vec<String>>,
}

fn default_required() -> bool {
    true
}

impl fmt::Display for ConfKitInteractiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfKitInteractiveType::Input => write!(f, "input"),
            ConfKitInteractiveType::Radio => write!(f, "radio"),
            ConfKitInteractiveType::Checkbox => write!(f, "checkbox"),
            ConfKitInteractiveType::Confirm => write!(f, "confirm"),
        }
    }
}
