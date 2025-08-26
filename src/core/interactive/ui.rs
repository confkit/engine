//! Author: xiaoYown
//! Created: 2025-07-22
//! Description: Interactive UI

use std::fmt;

// 所有交互式菜单的 UI
pub enum InteractiveUI {
    Main,
    Run,
    Builder,
    Image,
    // Log,
}

// 主菜单 UI
pub enum InteractiveMainUI {
    Run,
    Builder,
    Image,
    Log,
    Quit,
}

impl fmt::Display for InteractiveMainUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveMainUI::Run => write!(f, "[RUN] Execute project build task"),
            InteractiveMainUI::Builder => {
                write!(f, "[BUILDER] Manage build images and environments")
            }
            InteractiveMainUI::Image => write!(f, "[IMAGE] Manage build images"),
            InteractiveMainUI::Log => write!(f, "[LOG] Manage log files"),
            InteractiveMainUI::Quit => write!(f, "[QUIT] Exit"),
        }
    }
}

// 镜像管理菜单 UI
pub enum InteractiveImageUI {
    List,
    Create,
    Remove,
    Back,
}

impl fmt::Display for InteractiveImageUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveImageUI::List => write!(f, "[LIST] List all builder images"),
            InteractiveImageUI::Create => write!(f, "[CREATE] Create a new builder image"),
            InteractiveImageUI::Remove => write!(f, "[REMOVE] Remove a builder image"),
            InteractiveImageUI::Back => write!(f, "[BACK] Back to main menu"),
        }
    }
}

// builder 管理菜单 UI
pub enum InteractiveBuilderUI {
    List,
    Start,
    Stop,
    Restart,
    Create,
    Remove,
    Back,
}

impl fmt::Display for InteractiveBuilderUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveBuilderUI::List => write!(f, "[LIST] List all builder containers"),
            InteractiveBuilderUI::Start => write!(f, "[START] Start a builder container"),
            InteractiveBuilderUI::Stop => write!(f, "[STOP] Stop a builder container"),
            InteractiveBuilderUI::Restart => write!(f, "[RESTART] Restart a builder container"),
            InteractiveBuilderUI::Create => write!(f, "[CREATE] Create a new builder container"),
            InteractiveBuilderUI::Remove => write!(f, "[REMOVE] Remove a builder container"),
            InteractiveBuilderUI::Back => write!(f, "[BACK] Back to main menu"),
        }
    }
}

// 通用 Yes/No/Back
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractiveYesNoUI {
    Yes,
    No,
}

impl fmt::Display for InteractiveYesNoUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveYesNoUI::Yes => write!(f, "Yes"),
            InteractiveYesNoUI::No => write!(f, "No"),
        }
    }
}

// 通用选项
pub enum InteractiveOptionUI {
    Back,
}

impl fmt::Display for InteractiveOptionUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveOptionUI::Back => write!(f, "[BACK] Back to main menu"),
        }
    }
}
