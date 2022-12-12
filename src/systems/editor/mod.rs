use crate::utils::state::GameState;
use bevy::{
	input::mouse::{MouseScrollUnit, MouseWheel},
	prelude::*,
	render::camera::ScalingMode,
};
use iyes_loopless::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(editor_mouse_scroll.run_in_state(GameState::InGame));
	}
}

pub fn editor_mouse_scroll(
	mut scroll: EventReader<MouseWheel>,
	mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
	for event in scroll.iter() {
		match event.unit {
			MouseScrollUnit::Line => {
				if let Ok(mut projection) = query.get_single_mut() {
					if let ScalingMode::FixedVertical(size) = projection.scaling_mode {
						projection.scaling_mode = ScalingMode::FixedVertical(size + event.y * 20.);
					}
				}
			}
			MouseScrollUnit::Pixel => {
				if let Ok(mut projection) = query.get_single_mut() {
					if let ScalingMode::FixedVertical(size) = projection.scaling_mode {
						projection.scaling_mode = ScalingMode::FixedVertical(size + event.y);
					}
				}
			}
		}
	}
}
