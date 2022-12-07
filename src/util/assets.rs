use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_spine::{Atlas, SkeletonData, SkeletonJson};

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
