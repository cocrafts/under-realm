mod helper;

use bevy::prelude::{App, Plugin};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(helper::init);
	}
}
