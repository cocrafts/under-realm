use crate::utils::card::{CardType, ClassType};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Card {
	pub id: String,
	pub name: String,
	pub kind: CardType,
	pub rarity: u8,
	pub class: ClassType,
}
