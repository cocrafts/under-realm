use bevy::prelude::*;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
	let ground_handle = asset_server.load("textures/ground.png");

	commands.spawn(SpriteBundle {
		texture: ground_handle,
		..default()
	});
}
