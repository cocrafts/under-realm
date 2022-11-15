use crate::internal::TemplateFragment;
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

#[derive(Clone, Copy, PartialEq)]
pub enum ActivationType {
	Summon,
	Death,
	Passive,
	Attack,
	Defense,
	Glory,
	Prefight,
	Postfight,
	Charge,
	Inspire,
	Banner,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InspireSource {
	Metal,
	Wood,
	Water,
	Fire,
	Earth,
	Dark,
	Light,
	Summon,
	Death,
	Spell,
	Skill,
}

#[derive(Component)]
pub struct Skill {
	pub template: Vec<TemplateFragment>,
	pub trigger: Option<ActivationType>,
	pub inspire: Option<InspireSource>,
}
