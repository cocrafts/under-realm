use bevy::{prelude::*, window::close_on_esc};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use systems::board::BoardPlugin;
use systems::developer::DeveloperPlugins;
use systems::hand::HandPlugin;
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
	let default_plugins = DefaultPlugins.set(WindowPlugin {
		window: WindowDescriptor {
			fit_canvas_to_parent: true,
			..default()
		},
		..default()
	});
	app.insert_resource(ClearColor(systems::util::config::CLEAR))
		.add_plugins(default_plugins)
		.add_plugins(DefaultPickingPlugins)
		.add_plugin(EguiPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(HandPlugin)
		.add_startup_system(systems::setup)
		.add_system(close_on_esc);

	#[cfg(feature = "reload")]
	app.add_plugins(DeveloperPlugins);

	app.run();
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
