use bevy::asset::Handle;
use bevy::prelude::{Color, VerticalAlign};
use bevy::text::{Font, HorizontalAlign, Text, TextAlignment, TextSection, TextStyle};
use serde::{Deserialize, Serialize};

const DEFAULT_COLOR: [f32; 4] = [1., 1., 1., 1.];
const DEFAULT_SIZE: f32 = 24.;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateFragment {
	pub text: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<[f32; 4]>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<f32>,
}

impl Default for TemplateFragment {
	fn default() -> Self {
		TemplateFragment {
			text: "?".to_string(),
			color: None,
			size: None,
		}
	}
}

pub trait Printable {
	fn to_text(&self, font: Handle<Font>) -> Text;
}

impl Printable for Vec<TemplateFragment> {
	fn to_text(&self, font: Handle<Font>) -> Text {
		let sections = self
			.iter()
			.map(|i| TextSection {
				style: TextStyle {
					font: font.clone(),
					color: Color::from(i.color.unwrap_or(DEFAULT_COLOR)),
					font_size: i.size.unwrap_or(DEFAULT_SIZE),
				},
				value: i.text.to_string(),
			})
			.collect();

		Text {
			sections,
			alignment: TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		}
	}
}
