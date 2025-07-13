use std::fmt;

// 所有交互式菜单的 UI
pub enum InteractiveUI {
    Main,
    Run,
    Builder,
    Image,
    ImageRemove,
    ImageCreate,
    Log,
    Quit,
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
            InteractiveMainUI::Quit => write!(f, "[QUIT] Exit program"),
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
    Create,
    Remove,
    Back,
}

impl fmt::Display for InteractiveBuilderUI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveBuilderUI::List => write!(f, "[LIST] List all builder containers"),
            InteractiveBuilderUI::Create => write!(f, "[CREATE] Create a new builder container"),
            InteractiveBuilderUI::Remove => write!(f, "[REMOVE] Remove a builder container"),
            InteractiveBuilderUI::Back => write!(f, "[BACK] Back to main menu"),
        }
    }
}
