use std::marker::PhantomData;

/// T is a dummy type parameter used for type safety to avoid
/// confusing identifiers of different types
#[derive(Debug, Clone)]
pub struct Identifier<T> {
	pub what: String,
	phantom: PhantomData<T>,
}

impl<T> Identifier<T> {
	pub fn new(what: String) -> Identifier<T> {
		Identifier { what, phantom: PhantomData::default() }
	}
}

pub mod channels;
pub mod messages;
pub mod formatting;
pub mod image;
pub mod users;
pub mod notification;
