use crate::utils::{
	assets::FontSet,
	attribute::{ActivationType, ElementalType, InspireSource},
	skill::*,
};
use bevy::prelude::{Color, Component, Text, TextSection, TextStyle};

#[derive(Component)]
pub struct Elemental(ElementalType);

#[derive(Component)]
pub struct Attack(pub usize);

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct Defense(pub usize);

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
