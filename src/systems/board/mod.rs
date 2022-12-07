use crate::util::assets::Skeletons;
use bevy::prelude::*;
use bevy_spine::SpineBundle;

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	commands
		.spawn(SpineBundle {
			skeleton: skeletons.board.clone(),
			transform: Transform::from_xyz(0., 28., 0.),
			..default()
		})
		// .insert(Board)
		.insert(Name::new("Board"));

	commands
		.spawn(SpineBundle {
			skeleton: skeletons.atmosphere.clone(),
			transform: Transform::from_xyz(0., 28., 1.0),
			..default()
		})
		// .insert(Atmosphere)
		.insert(Name::new("Atmosphere"));
}
