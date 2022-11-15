use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_spine::SpinePlugin;
use systems::{BoardPlugin, DeckPlugin, EditorPlugin, HandPlugin};
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
	use bevy::prelude::*;
	pub use components::*;
	hot_functions_from_file!("systems/src/lib.rs");
}

fn main() {
	let mut app = App::new();
	let plugins = DefaultPlugins.set(WindowPlugin {
		window: WindowDescriptor {
			position: WindowPosition::At(Vec2::new(10., 300.)),
			width: 800.,
			height: 500.,
			fit_canvas_to_parent: true,
			..default()
		},
		..default()
	});
	app.insert_resource(ClearColor(systems::util::config::CLEAR))
		.add_plugins(plugins)
		.add_plugin(SpinePlugin)
		.add_plugin(EguiPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(DeckPlugin)
		.add_plugin(HandPlugin);

	#[cfg(feature = "reload")]
	app.add_plugin(WorldInspectorPlugin::new())
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugin(EditorPlugin);

	app.add_startup_system(systems::setup)
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
