use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq)]
pub enum CardType {
	Hero,
	Troop,
	Spell,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClassType {
	Tanker,
	Warrior,
	Assassin,
	Wizard,
	Summoner,
}

#[derive(Component)]
pub struct Card {
	pub id: String,
	pub kind: CardType,
	pub rarity: usize,
}

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;
