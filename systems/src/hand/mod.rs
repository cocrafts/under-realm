mod helper;

use bevy::prelude::{App, Plugin};

pub struct HandPlugin;
impl Plugin for HandPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(helper::init);
	}
}
