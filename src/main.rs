mod components;
mod systems;
mod utils;

use crate::utils::assets::{FontAssets, LoadingAssets, SpineAssets, TextureAssets};
#[cfg(feature = "dynamic")]
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::close_on_esc};
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
#[cfg(feature = "dynamic")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_spine::SpinePlugin;
use iyes_loopless::prelude::*;
use systems::{
	asset, board::BoardPlugin, card::CardPlugin, editor::EditorPlugin, loading::LoadingPlugin,
	tower::TowerPlugin,
};
use utils::{config, state::*};
use wasm_bindgen::prelude::*;

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
				.with_collection::<TextureAssets>()
				.with_collection::<SpineAssets>(),
		)
		.add_plugins(defaults)
		.add_plugin(SpinePlugin)
		.add_plugin(EguiPlugin)
		.add_plugin(LoadingPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(CardPlugin)
		.add_plugin(TowerPlugin);

	#[cfg(feature = "dynamic")]
	app.add_plugin(WorldInspectorPlugin::new())
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugin(EditorPlugin);

	app.init_collection::<LoadingAssets>()
		.init_collection::<FontAssets>()
		.add_enter_system(GameState::Setup, asset::configure)
		.add_enter_system(GameState::Duel, asset::duel)
		.add_system(close_on_esc)
		.run();
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_u32(a: u32);

	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	log(&format!("Hello {}, from inside Rust!!!", name));
}
