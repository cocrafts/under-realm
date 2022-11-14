use bevy::{
	prelude::*,
	render::mesh::{shape::Quad, Mesh},
};

const GROUND_WIDTH: f32 = 8.0;
const GROUND_HEIGHT: f32 = GROUND_WIDTH * 0.59640103;

pub fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let quad_mesh = Mesh::from(Quad::new(Vec2::new(GROUND_WIDTH, GROUND_HEIGHT)));
	let img_handle = asset_server.load("textures/ground.png");
	let quad_handle = meshes.add(quad_mesh);
	let material_handle = materials.add(StandardMaterial {
		base_color_texture: Some(img_handle.clone()),
		perceptual_roughness: 1.0,
		alpha_mode: AlphaMode::Blend,
		..default()
	});

	commands.spawn(PbrBundle {
		mesh: quad_handle,
		material: material_handle,
		..default()
	});
}
