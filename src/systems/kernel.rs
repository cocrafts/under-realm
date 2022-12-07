use crate::util::assets::{Skeletons, SpineAssets};
use crate::util::state::GameState;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_spine::prelude::*;
use iyes_loopless::state::NextState;

pub fn setup(
	mut commands: Commands,
	mut skeletons: ResMut<Assets<SkeletonData>>,
	spines: Res<SpineAssets>,
) {
	let game_skeletons = Skeletons {
		card: skeletons.add(SkeletonData::new_from_json(
			spines.card_json.clone(),
			spines.card_atlas.clone(),
		)),
		tower: skeletons.add(SkeletonData::new_from_json(
			spines.tower_json.clone(),
			spines.tower_atlas.clone(),
		)),
		board: skeletons.add(SkeletonData::new_from_json(
			spines.board_json.clone(),
			spines.board_atlas.clone(),
		)),
		atmosphere: skeletons.add(SkeletonData::new_from_json(
			spines.atmosphere_json.clone(),
			spines.atmosphere_atlas.clone(),
		)),
	};

	commands.insert_resource(game_skeletons);
	commands
		.spawn(Camera2dBundle {
			camera: Camera { ..default() },
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::FixedVertical(1100.),
				..default()
			},
			..default()
		})
		.insert(Name::new("Primary Camera"));

	commands.insert_resource(NextState(GameState::Duel));
}

pub fn duel() {
	println!("hmm");
}
