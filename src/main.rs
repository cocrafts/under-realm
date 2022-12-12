mod components;
mod systems;
mod utils;

use crate::utils::assets::{FontAssets, LoadingAssets, SpineAssets, TextureAssets};
use bevy::diagnostic::Diagnostics;
#[cfg(feature = "dynamic")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_asset_loader::prelude::*;
#[cfg(feature = "dynamic")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_spine::SpinePlugin;
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};
#[cfg(feature = "dynamic")]
use systems::editor::EditorPlugin;
use systems::{
	asset, board::BoardPlugin, card::CardPlugin, loading::LoadingPlugin, tower::TowerPlugin,
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
		.add_loopless_state(GameState::AssetLoading)
		.add_loading_state(
			LoadingState::new(GameState::AssetLoading)
				.with_collection::<TextureAssets>()
				.with_collection::<SpineAssets>()
				.with_collection::<FontAssets>(),
		)
		.add_plugins(defaults)
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.add_plugin(ProgressPlugin::new(GameState::AssetLoading).continue_to(GameState::Splash))
		.add_plugin(SpinePlugin)
		.add_plugin(LoadingPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(CardPlugin)
		.add_plugin(TowerPlugin);

	#[cfg(feature = "dynamic")]
	app.add_plugin(WorldInspectorPlugin::new())
		.add_plugin(EditorPlugin);

	app.init_collection::<LoadingAssets>()
		.add_enter_system(GameState::Splash, asset::configure)
		.add_enter_system(GameState::InGame, asset::duel)
		.add_system(
			track_fake_long_task
				.run_in_state(GameState::AssetLoading)
				.before("print"),
		)
		.add_system(print_progress.label("print"))
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

const DURATION_LONG_TASK_IN_SECS: f64 = 5.0;

fn track_fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
	if time.elapsed_seconds_f64() > DURATION_LONG_TASK_IN_SECS {
		info!("Long task is completed");
		progress.manually_track(true.into());
	} else {
		progress.manually_track(false.into());
	}
}

fn print_progress(
	progress: Option<Res<ProgressCounter>>,
	diagnostics: Res<Diagnostics>,
	mut last_done: Local<u32>,
) {
	if let Some(progress) = progress.map(|counter| counter.progress()) {
		if progress.done > *last_done {
			*last_done = progress.done;
			info!(
				"[Frame {}] Changed progress: {:?}",
				diagnostics
					.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
					.map(|diagnostic| diagnostic.value().unwrap_or(0.))
					.unwrap_or(0.),
				progress
			);
		}
	}
}
