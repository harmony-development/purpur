/// A basic widget system is available for displaying things like dialogs and sidebars and menus.
/// When making changes to widgets, the following platforms should be considered:
/// - Mobile
/// - Desktop
/// - Terminal UIs
///
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Button {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    sensitive: bool,
    // TODO: consider i18n
    placeholder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Widget {
    Input(Input),
    Button(Button),
}

/// This should be rendered as a set of UI elements displayed side by side.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    label: Option<String>,
    widgets: Vec<Widget>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dialog {
    title: String,
    /// Set of widgets to display
    widgets: Vec<Row>,
    /// On mobile, these correspond to buttons such as "Submit" and "Cancel", etc.
    /// These are a separate field from widgets for accessibility reasons and for clients to have
    /// more control over the rendering.
    actions: Vec<Button>,
}
