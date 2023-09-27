use super::Identifier;

#[derive(Debug, Clone)]
pub struct Notification {
	pub id: Identifier<Notification>,
	pub title: String,
	pub body: String,
}