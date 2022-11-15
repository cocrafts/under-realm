use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::{Camera, EventReader, OrthographicProjection, Query, With};
use bevy::render::camera::ScalingMode;

pub fn editor_mouse_scroll(
	mut scroll: EventReader<MouseWheel>,
	mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
	for event in scroll.iter() {
		match event.unit {
			MouseScrollUnit::Line => {
				println!(
					"Scroll (line units): vertical: {}, horizontal: {}",
					event.y, event.x
				);
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
