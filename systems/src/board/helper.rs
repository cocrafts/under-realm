use crate::util::{Fonts, GameTextures, Skeletons};
use bevy::prelude::*;
use bevy_spine::SpineBundle;
use components::internal::{Printable, TemplateFragment};

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	commands
		.spawn(SpineBundle {
			skeleton: skeletons.board.clone(),
			transform: Transform::from_xyz(0., 28., 0.),
			..default()
		})
		.insert(Name::new("Board"));
}
