use bevy::prelude::*;
use bevy_spine::{prelude::*, rusty_spine::Skin};
use components::{FromEnemy, Skeletons};
use components::{FromPlayer, Tower};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, init)
			.add_system(player_tower_spawned)
			.add_system(enemy_tower_spawned);
	}
}

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	commands
		.spawn(SpineBundle {
			skeleton: skeletons.tower.clone(),
			transform: Transform::from_xyz(580., 370., 0.2),
			..default()
		})
		.insert(Tower)
		.insert(FromEnemy)
		.insert(Name::new("Tower"));

	commands
		.spawn(SpineBundle {
			skeleton: skeletons.tower.clone(),
			transform: Transform::from_xyz(-630., -400., 0.2),
			..default()
		})
		.insert(Tower)
		.insert(FromPlayer)
		.insert(Name::new("Tower"));
}

pub fn player_tower_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut tower_spines: Query<&mut Spine, (With<Tower>, With<FromPlayer>)>,
) {
	for _ in spine_ready_event.iter() {
		for mut spine in tower_spines.iter_mut() {
			let skin = {
				let skeleton_data = spine.skeleton.data();
				let mut skin = Skin::new("custom");
				skin.add_skin(skeleton_data.find_skin("base/blue").unwrap().as_ref());
				skin
			};

			spine.skeleton.set_skin(&skin);
		}
	}
}

pub fn enemy_tower_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut tower_spines: Query<&mut Spine, (With<Tower>, With<FromEnemy>)>,
) {
	for _ in spine_ready_event.iter() {
		for mut spine in tower_spines.iter_mut() {
			let skin = {
				let skeleton_data = spine.skeleton.data();
				let mut skin = Skin::new("custom");
				skin.add_skin(skeleton_data.find_skin("base/red").unwrap().as_ref());
				skin
			};

			spine.skeleton.set_skin(&skin);
		}
	}
}
