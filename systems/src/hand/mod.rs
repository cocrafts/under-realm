mod helper;

use bevy::app::StartupStage;
use bevy::prelude::{App, Plugin};

pub struct HandPlugin;
impl Plugin for HandPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, helper::init);
	}
}
