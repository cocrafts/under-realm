pub mod board;
pub mod developer;
pub mod hand;
pub mod util;

use bevy::{
	prelude::*,
	render::mesh::{shape::UVSphere, Mesh},
};
use bevy_mod_picking::PickingCameraBundle;

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn(PointLightBundle {
			transform: Transform::from_xyz(0.0, 0.0, 6.0),
			point_light: PointLight {
				intensity: 2000.0,
				color: Color::rgb(1.0, 0.86, 0.85),
				shadows_enabled: true,
				..default()
			},
			..default()
		})
		.with_children(|builder| {
			builder.spawn(PbrBundle {
				mesh: meshes.add(Mesh::from(UVSphere {
					radius: 0.1,
					..default()
				})),
				material: materials.add(StandardMaterial {
					base_color: Color::GREEN,
					emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
					..default()
				}),
				..default()
			});
		})
		.insert(Name::new("LightOrb"));

	commands
		.spawn(Camera3dBundle {
			transform: Transform::from_xyz(0.0, 0.0, 5.7).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		})
		.insert(PickingCameraBundle::default())
		.insert(Name::new("Camera"));
}
