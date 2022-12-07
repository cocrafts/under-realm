use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_spine::{Atlas, SkeletonData, SkeletonJson};

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

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
	#[asset(path = "fonts/Vollkorn-Regular.ttf")]
	pub vollkorn_regular: Handle<Font>,
	#[asset(path = "fonts/Vollkorn-Bold.ttf")]
	pub vollkorn_medium: Handle<Font>,
	#[asset(path = "fonts/Vollkorn-Black.ttf")]
	pub vollkorn_bold: Handle<Font>,
	#[asset(path = "fonts/FiraSansCondensed-Regular.ttf")]
	pub fira_regular: Handle<Font>,
	#[asset(path = "fonts/FiraSansCondensed-Medium.ttf")]
	pub fira_medium: Handle<Font>,
	#[asset(path = "fonts/FiraSansCondensed-Bold.ttf")]
	pub fira_bold: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
	#[asset(path = "textures/gem.png")]
	pub gem: Handle<Image>,
}

#[derive(Resource)]
pub struct Skeletons {
	pub card: Handle<SkeletonData>,
	pub tower: Handle<SkeletonData>,
	pub board: Handle<SkeletonData>,
	pub atmosphere: Handle<SkeletonData>,
}

#[derive(AssetCollection, Resource)]
pub struct SpineAssets {
	#[asset(path = "spines/card.json")]
	pub card_json: Handle<SkeletonJson>,
	#[asset(path = "spines/card.atlas")]
	pub card_atlas: Handle<Atlas>,
	#[asset(path = "spines/tower.json")]
	pub tower_json: Handle<SkeletonJson>,
	#[asset(path = "spines/tower.atlas")]
	pub tower_atlas: Handle<Atlas>,
	#[asset(path = "spines/board.json")]
	pub board_json: Handle<SkeletonJson>,
	#[asset(path = "spines/board.atlas")]
	pub board_atlas: Handle<Atlas>,
	#[asset(path = "spines/atmosphere.json")]
	pub atmosphere_json: Handle<SkeletonJson>,
	#[asset(path = "spines/atmosphere.atlas")]
	pub atmosphere_atlas: Handle<Atlas>,
}
