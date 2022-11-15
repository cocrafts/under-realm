mod helper;

use bevy::app::StartupStage;
use bevy::prelude::{App, Plugin};

pub struct DeckPlugin;
impl Plugin for DeckPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(
			StartupStage::PostStartup,
			helper::distribute_starting_cards,
		);
	}
}
