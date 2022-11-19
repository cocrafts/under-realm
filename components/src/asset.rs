use bevy::prelude::*;
use bevy_spine::SkeletonData;

#[derive(Resource)]
pub struct GameTextures {
	pub tower_blue: Handle<Image>,
	pub tower_red: Handle<Image>,
}

#[derive(Resource)]
pub struct Fonts {
	pub vollkorn: Handle<Font>,
	pub vollkorn_bold: Handle<Font>,
}

#[derive(Resource)]
pub struct Skeletons {
	pub card: Handle<SkeletonData>,
	pub board: Handle<SkeletonData>,
	pub tower: Handle<SkeletonData>,
}
