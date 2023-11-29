/// A basic widget system is available for displaying things like dialogs and sidebars and menus.
/// When making changes to widgets, the following platforms should be considered:
/// - Mobile
/// - Desktop
/// - Terminal UIs
///

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Widget {
    Input {
        sensitive: bool,
        // TODO: consider i18n
        placeholder: String,
    },
    Button {
        text: String,
    },
}

/// This should be rendered as a set of UI elements displayed side by side.
///
#[derive(Serialize, Deserialize)]
pub struct Row {
    label: Option<String>,
    widgets: Vec<Widget>,
}
