pub mod board;
pub mod deck;
pub mod editor;
pub mod hand;
pub mod util;

pub use crate::board::BoardPlugin;
pub use crate::deck::DeckPlugin;
pub use crate::editor::EditorPlugin;
pub use crate::hand::HandPlugin;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_spine::SkeletonData;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut skeletons: ResMut<Assets<SkeletonData>>,
) {
	let game_skeletons = util::Skeletons {
		board: skeletons.add(SkeletonData::new_from_json(
			asset_server.load("spines/board.json"),
			asset_server.load("spines/board.atlas"),
		)),
	};

	let game_textures = util::GameTextures {
		tower_blue: asset_server.load("textures/tower-blue.png"),
		tower_red: asset_server.load("textures/tower-red.png"),
	};

	let game_fonts = util::Fonts {
		vollkorn: asset_server.load("fonts/Vollkorn-Regular.ttf"),
		vollkorn_bold: asset_server.load("fonts/Vollkorn-SemiBold.ttf"),
	};

	commands.insert_resource(game_skeletons);
	commands.insert_resource(game_textures);
	commands.insert_resource(game_fonts);
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
}
