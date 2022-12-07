mod components;
mod systems;
mod utils;

use crate::utils::assets::{FontAssets, SpineAssets, TextureAssets};
#[cfg(feature = "dynamic")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
#[cfg(feature = "dynamic")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_spine::SpinePlugin;
use iyes_loopless::prelude::*;
use systems::{asset, board::BoardPlugin, card::CardPlugin, editor::EditorPlugin};
use utils::{config, state::*};

fn main() {
	let mut app = App::new();
	let defaults = DefaultPlugins.set(WindowPlugin {
		window: WindowDescriptor {
			position: WindowPosition::At(Vec2::new(10., 300.)),
			width: 1200.,
			height: 780.,
			fit_canvas_to_parent: true,
			..default()
		},
		..default()
	});

	app.insert_resource(ClearColor(config::CLEAR))
		.add_loopless_state(GameState::Loading)
		.add_loading_state(
			LoadingState::new(GameState::Loading)
				.continue_to_state(GameState::Setup)
				.with_collection::<FontAssets>()
				.with_collection::<TextureAssets>()
				.with_collection::<SpineAssets>(),
		)
		.add_plugins(defaults)
		.add_plugin(SpinePlugin)
		.add_plugin(EguiPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(CardPlugin);

	#[cfg(feature = "dynamic")]
	app.add_plugin(WorldInspectorPlugin::new())
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugin(EditorPlugin);

	app.add_enter_system(GameState::Setup, asset::configure)
		.add_enter_system(GameState::Duel, asset::duel);

	app.run();
}
