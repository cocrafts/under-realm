use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct GameState {
	pub round: usize,
}

#[derive(Resource)]
pub struct GameConfig {
	pub starting_card_count: usize,
	pub max_unit: usize,
}
