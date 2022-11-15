use crate::util::GameTextures;
use bevy::prelude::*;

pub fn init(mut commands: Commands, game_textures: Res<GameTextures>) {
	commands.spawn(SpriteBundle {
		texture: game_textures.ground.clone(),
		..default()
	});
}
