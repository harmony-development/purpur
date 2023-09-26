use super::image::Image;

/// The style that the child channels get rendered as
#[derive(Debug, Clone)]
pub enum RenderStyle {
    /// Renders as a icon with a tooltip when hovered
    IconsOnly,
    /// Renders only the channel's text
    TextOnly,
    /// Bool corresponds to "reversing" the text and icon order.
    /// Icon comes last when set to true
    IconsAndText(bool),
}

/// The placement of the child channel list
#[derive(Debug, Clone)]
pub enum ChannelPlacement {
    /// Place the child channels "under" the current channel
    Under,
    /// Places the child channels to the left or right of the parent's channel siblings
    /// True = left, false = right
    Side(bool),
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub image: Image,
    pub children: Option<Vec<Channel>>,
    pub preferred_render_style: RenderStyle,
    pub placement: ChannelPlacement,
}
