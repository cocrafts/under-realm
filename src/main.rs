use bevy::prelude::*;
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

	app.insert_resource(WindowDescriptor {
		fit_canvas_to_parent: true,
		..default()
	})
	.insert_resource(ClearColor(Color::rgb(0.24313725, 0.11764706, 0.08627451)))
	.add_plugins(DefaultPlugins)
	.add_startup_system(systems::setup)
	.add_startup_system(network_system)
	.add_system_set(
		SystemSet::new()
			.with_system(player_movement_system)
			.with_system(player_shooting_system)
			.with_system(bullet_movement_system)
			.with_system(bullet_hit_system)
			.with_system(spawn_other_ships)
			.with_system(move_other_ships),
	)
	.add_system(bevy::window::close_on_esc);

	#[cfg(feature = "debug")]
	app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
	#[cfg(feature = "debug")]
	app.add_plugin(bevy_diagnostic_visualizer::DiagnosticVisualizerPlugin::default);

	app.run();
}

fn network_system() {}

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	// The `console.log` is quite polymorphic, so we can bind it with multiple
	// signatures. Note that we need to use `js_name` to ensure we always call
	// `log` in JS.
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_u32(a: u32);

	// Multiple arguments too!
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	log(&format!("Hello {}, from inside Rust!!!", name));
}
