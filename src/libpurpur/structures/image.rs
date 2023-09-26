/// An image object
#[derive(Debug, Clone)]
pub enum Image {
    /// Url resource that gets cached clientside automatically when first requested
    Url(String),
    /// Raw bytes representing the image data
    Bytes(Vec<u8>),
}