use crate::utils::assets::FontSet;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const DEFAULT_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.];
pub const DEFAULT_SIZE: f32 = 22.;

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
	fn to_text(&self, font: FontSet) -> Text;
}

impl Printable for Vec<TemplateFragment> {
	fn to_text(&self, font: FontSet) -> Text {
		let sections = self
			.iter()
			.map(|i| TextSection {
				style: TextStyle {
					font: font.medium.clone(),
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
