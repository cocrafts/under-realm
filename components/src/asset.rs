use bevy::prelude::*;
use bevy_spine::SkeletonData;

#[derive(Resource)]
pub struct GameTextures {
	pub tower_blue: Handle<Image>,
	pub tower_red: Handle<Image>,
}

#[derive(Clone)]
pub struct FontSet {
	pub regular: Handle<Font>,
	pub medium: Handle<Font>,
	pub bold: Handle<Font>,
}

#[derive(Resource)]
pub struct Fonts {
	pub vollkorn: FontSet,
	pub fira: FontSet,
}

#[derive(Resource)]
pub struct Skeletons {
	pub card: Handle<SkeletonData>,
	pub board: Handle<SkeletonData>,
	pub tower: Handle<SkeletonData>,
}
