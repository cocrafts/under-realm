pub mod board;
pub mod deck;
pub mod editor;
pub mod hand;
pub mod tower;

pub use crate::board::BoardPlugin;
pub use crate::deck::DeckPlugin;
pub use crate::editor::EditorPlugin;
pub use crate::hand::HandPlugin;
pub use crate::tower::TowerPlugin;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_spine::prelude::*;
use components::*;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut skeletons: ResMut<Assets<SkeletonData>>,
) {
	let game_skeletons = Skeletons {
		card: skeletons.add(SkeletonData::new_from_json(
			asset_server.load("spines/card.json"),
			asset_server.load("spines/card.atlas"),
		)),
		board: skeletons.add(SkeletonData::new_from_json(
			asset_server.load("spines/board.json"),
			asset_server.load("spines/board.atlas"),
		)),
		tower: skeletons.add(SkeletonData::new_from_json(
			asset_server.load("spines/tower.json"),
			asset_server.load("spines/tower.atlas"),
		)),
	};

	let game_textures = GameTextures {
		tower_blue: asset_server.load("textures/tower-blue.png"),
		tower_red: asset_server.load("textures/tower-red.png"),
	};

	let game_fonts = Fonts {
		vollkorn: FontSet {
			regular: asset_server.load("fonts/Vollkorn-Regular.ttf"),
			medium: asset_server.load("fonts/Vollkorn-Regular.ttf"),
			bold: asset_server.load("fonts/Vollkorn-Bold.ttf"),
		},
		fira: FontSet {
			regular: asset_server.load("fonts/FiraSansCondensed-Regular.ttf"),
			medium: asset_server.load("fonts/FiraSansCondensed-Medium.ttf"),
			bold: asset_server.load("fonts/FiraSansCondensed-Bold.ttf"),
		},
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

#[no_mangle]
pub fn hot_debug(keys: Res<Input<KeyCode>>, mut towers: Query<&mut Spine, With<Tower>>) {
	if keys.just_pressed(KeyCode::Space) {
		for mut spine in towers.iter_mut() {
			let mut rng = thread_rng();
			let skin = {
				let skeleton_data = spine.skeleton.data();
				let mut skin = Skin::new("custom");
				let skin_type = ["base/red", "base/blue"].choose(&mut rng).unwrap();
				skin.add_skin(skeleton_data.find_skin(skin_type).unwrap().as_ref());
				skin
			};

			spine.skeleton.set_skin(&skin);
			println!("spine {:?}!", skin);
		}
	}
}
