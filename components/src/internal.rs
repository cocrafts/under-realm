use bevy::asset::Handle;
use bevy::prelude::Color;
use bevy::text::{Font, Text, TextSection, TextStyle};
use bevy::utils::default;
use serde::{Deserialize, Serialize};

const DEFAULT_COLOR: [f32; 4] = [1., 1., 1., 1.];
const DEFAULT_SIZE: f32 = 24.;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateFragment {
	pub text: String,
	#[serde(default = "default_color")]
	pub color: [f32; 4],
	#[serde(default = "default_size")]
	pub size: f32,
}

impl Default for TemplateFragment {
	fn default() -> Self {
		TemplateFragment {
			text: "?".to_string(),
			color: DEFAULT_COLOR,
			size: DEFAULT_SIZE,
		}
	}
}

fn default_color() -> [f32; 4] {
	DEFAULT_COLOR
}

fn default_size() -> f32 {
	DEFAULT_SIZE
}

pub trait Printable {
	fn to_text(&self, font: Handle<Font>) -> Text;
}

impl Printable for Vec<TemplateFragment> {
	fn to_text(&self, font: Handle<Font>) -> Text {
		let sections = self
			.into_iter()
			.map(|i| TextSection {
				style: TextStyle {
					font: font.clone(),
					color: Color::from(i.color),
					font_size: i.size,
				},
				value: i.text.to_string(),
				..default()
			})
			.collect();

		Text {
			sections,
			..default()
		}
	}
}
