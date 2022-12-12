use crate::components::LoadingElement;
use crate::utils::{assets::*, state::GameState};
use bevy::{prelude::*, render::camera::ScalingMode, text::Text2dBounds};
use iyes_loopless::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
	fn build(&self, app: &mut App) {
		app.add_enter_system(GameState::AssetLoading, init)
			.add_exit_system(GameState::AssetLoading, exit);
	}
}

pub fn init(mut commands: Commands, assets: Res<LoadingAssets>) {
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

	commands
		.spawn(SpriteBundle {
			texture: assets.bg.clone(),
			transform: Transform {
				translation: Vec3::new(0., 0., 0.),
				rotation: Quat::default(),
				scale: Vec3::new(1.1, 1.1, 1.),
			},
			..default()
		})
		.insert(Name::new("Loading Background"))
		.insert(LoadingElement);

	commands
		.spawn(SpriteBundle {
			texture: assets.frame.clone(),
			transform: Transform {
				translation: Vec3::new(0., 0., 0.),
				rotation: Quat::default(),
				scale: Vec3::new(1.1, 1.1, 1.),
			},
			sprite: Sprite {
				color: Color::rgba(0.45, 0.254, 0.254, 0.5),
				..default()
			},
			..default()
		})
		.insert(Name::new("Loading Frame"))
		.insert(LoadingElement);

	commands
		.spawn(SpriteBundle {
			texture: assets.logo.clone(),
			transform: Transform {
				translation: Vec3::new(-540., -180., 0.),
				rotation: Quat::default(),
				scale: Vec3::new(0.52, 0.52, 1.),
			},
			..default()
		})
		.insert(Name::new("Loading Background"))
		.insert(LoadingElement);

	commands
		.spawn(Text2dBundle {
			text: Text::from_section(
				"Under Realm: Rise of Magic takes place in a chaotic, fragmented world of ATEM where human and other races are constantly fighting each other, to wrench the endless thirst for power, wealth, and gradually take control over ATEM.",
				TextStyle {
					color: Color::rgba(1., 0.8, 0.8, 0.8),
					font: assets.fira_regular.clone(),
					font_size: 18.,
				},
			),
			text_2d_bounds: Text2dBounds {
				size: Vec2::new(740., 300.),
			},
			transform: Transform::from_xyz(-740., -270., 0.1),
			..default()
		})
		.insert(Name::new("Loading Intro"))
		.insert(LoadingElement);
}

pub fn exit(mut commands: Commands, entities: Query<Entity, With<LoadingElement>>) {
	for entity in entities.iter() {
		commands.entity(entity).despawn();
	}
}
