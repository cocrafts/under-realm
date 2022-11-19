use bevy::app::StartupStage;
use bevy::prelude::{App, Plugin};

pub struct DeckPlugin;
impl Plugin for DeckPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, distribute_starting_cards);
	}
}

pub fn distribute_starting_cards() {
	println!("This is Init, from hand Plugin");
}
