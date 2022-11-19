use bevy::prelude::*;
use bevy_spine::SpineBundle;
use components::Skeletons;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, init);
	}
}

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	commands
		.spawn(SpineBundle {
			skeleton: skeletons.board.clone(),
			transform: Transform::from_xyz(0., 28., 0.),
			..default()
		})
		.insert(Name::new("Board"));

	commands
		.spawn(SpineBundle {
			skeleton: skeletons.card.clone(),
			transform: Transform::from_xyz(0., 0., 0.1),
			..default()
		})
		.insert(Name::new("Card"));
}
