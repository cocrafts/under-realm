pub mod util;

use bevy::{
	prelude::*,
	render::{
		camera::ScalingMode,
		mesh::{
			shape::{Quad, UVSphere},
			Mesh,
		},
	},
	sprite::collide_aabb,
};
use components::*;
use rand::{thread_rng, Rng};

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let ground_handle = asset_server.load("textures/surface.png");
	let quad_width = 8.0;
	let quad_height = quad_width * 0.59640103;
	let quad_mesh = Mesh::from(Quad::new(Vec2::new(quad_width, quad_height)));
	let quad_handle = meshes.add(quad_mesh);
	let material_handle = materials.add(StandardMaterial {
		base_color_texture: Some(ground_handle.clone()),
		perceptual_roughness: 1.0,
		alpha_mode: AlphaMode::Blend,
		..default()
	});

	commands
		.spawn(PbrBundle {
			mesh: quad_handle,
			material: material_handle,
			..default()
		})
		.insert(Name::new("Ground"));

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
			builder.spawn_bundle(PbrBundle {
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
		.insert(Name::new("Camera"));
}
