use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq)]
pub enum ElementalType {
	Metal,
	Wood,
	Water,
	Fire,
	Earth,
	Dark,
	Light,
}

#[derive(Component)]
pub struct Elemental(ElementalType);

#[derive(Component)]
pub struct Attack(usize);

#[derive(Component)]
pub struct Health(usize);

#[derive(Component)]
pub struct Defense(usize);
