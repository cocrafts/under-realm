use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineReadyEvent};
use components::{Atmosphere, Board, Skeletons};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, init)
			.add_system(atmosphere_spawned);
	}
}

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	commands
		.spawn(SpineBundle {
			skeleton: skeletons.board.clone(),
			transform: Transform::from_xyz(0., 28., 0.),
			..default()
		})
		.insert(Board)
		.insert(Name::new("Board"));

	commands
		.spawn(SpineBundle {
			skeleton: skeletons.atmosphere.clone(),
			transform: Transform::from_xyz(0., 28., 1.0),
			..default()
		})
		.insert(Atmosphere)
		.insert(Name::new("Atmosphere"));
}

pub fn atmosphere_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut spine_query: Query<(&mut Spine, &Atmosphere)>,
) {
	for event in spine_ready_event.iter() {
		if let Ok((mut spine, _)) = spine_query.get_mut(event.entity) {
			if let Some(mut slot) = spine.skeleton.find_slot_mut("lightbeam") {
				slot.color_mut().set_a(0.1);
			}
		}
	}
}
