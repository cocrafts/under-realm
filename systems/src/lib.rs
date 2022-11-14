pub mod board;
pub mod hand;
pub mod util;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn setup(mut commands: Commands) {
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
