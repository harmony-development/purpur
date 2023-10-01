use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

/// T is a dummy type parameter used for type safety to avoid
/// confusing identifiers of different types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier<T> {
    pub what: String,
    #[serde(skip_serializing, default)]
    phantom: PhantomData<T>,
}

impl<T> Identifier<T> {
    pub fn new(what: String) -> Identifier<T> {
        Identifier {
            what,
            phantom: PhantomData::default(),
        }
    }
}

pub mod channels;
pub mod formatting;
pub mod image;
pub mod messages;
pub mod notification;
pub mod users;
