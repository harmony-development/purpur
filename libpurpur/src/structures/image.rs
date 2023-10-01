use serde::{Deserialize, Serialize};

/// An image object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Image {
    /// Url resource that gets cached clientside automatically when first requested
    Url(String),
    /// Raw bytes representing the image data
    Bytes(Vec<u8>),
}
