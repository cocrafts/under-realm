use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
	pub ground: Handle<Image>,
}

#[derive(Resource)]
pub struct Fonts {
	pub vollkorn: Handle<Font>,
	pub vollkorn_bold: Handle<Font>,
}
