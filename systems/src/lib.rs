pub mod board;
pub mod hand;
pub mod util;

use crate::util::GameTextures;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let game_textures = GameTextures {
		ground: asset_server.load("textures/ground.png"),
	};

	commands.insert_resource(game_textures);
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				hdr: true,
				..default()
			},
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::FixedVertical(900.),
				..default()
			},
			..default()
		},
		BloomSettings::default(),
	));
}
