use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct GameState {
	round: usize,
}

#[derive(Resource)]
pub struct GameConfig {
	starting_card_count: usize,
	max_unit: usize,
}
