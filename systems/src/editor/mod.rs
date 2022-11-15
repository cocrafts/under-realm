mod mouse;

use bevy::prelude::{App, Plugin};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(mouse::editor_mouse_scroll);
	}
}
