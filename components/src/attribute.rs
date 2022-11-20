use crate::asset::FontSet;
use crate::internal::{Printable, TemplateFragment, DEFAULT_COLOR, DEFAULT_SIZE};
use bevy::prelude::*;

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
	pub activation: Option<ActivationType>,
	pub charge: Option<u8>,
	pub inspire: Option<InspireSource>,
}

impl Printable for Skill {
	fn to_text(&self, font: FontSet) -> Text {
		let font_bold = font.bold.clone();
		let mut template_text = self.template.to_text(font);
		template_text.sections.insert(
			0,
			TextSection {
				value: "Prepend: ".to_string(),
				style: TextStyle {
					font: font_bold,
					color: Color::from(DEFAULT_COLOR),
					font_size: DEFAULT_SIZE,
				},
			},
		);
		template_text
	}
}
