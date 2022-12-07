use bevy::{
	prelude::*,
	text::{Text, Text2dBounds, Text2dBundle},
};

pub fn attach_gems(commands: &mut Commands, entity: Entity, texture: &Handle<Image>, rarity: u8) {
	let gem_count = rarity / 3;

	for level in 0..gem_count {
		commands.entity(entity).with_children(|parent| {
			parent.spawn(SpriteBundle {
				texture: texture.clone(),
				transform: Transform {
					translation: Vec3 {
						x: level as f32 * 22.,
						y: 0.,
						z: 0.1,
					},
					scale: Vec3 {
						x: 0.4,
						y: 0.4,
						z: 1.,
					},
					..default()
				},
				..default()
			});
		});
	}
}

pub fn inject_skill(commands: &mut Commands, entity: Entity, text: Text) {
	commands.entity(entity).with_children(|parent| {
		parent.spawn(Text2dBundle {
			text,
			transform: Transform::from_xyz(0., 0., 0.1),
			text_2d_bounds: Text2dBounds {
				size: Vec2::new(280.0, 200.0),
			},
			..default()
		});
	});
}

pub fn inject_name(
	commands: &mut Commands,
	entity: Entity,
	value: impl Into<String>,
	font: Handle<Font>,
) {
	commands.entity(entity).with_children(|parent| {
		parent.spawn(Text2dBundle {
			text: Text::from_section(
				value,
				TextStyle {
					color: Color::from([0.1, 0.1, 0.1, 1.0]),
					font,
					font_size: 28.,
				},
			)
			.with_alignment(TextAlignment::CENTER),
			transform: Transform::from_xyz(0., 0., 0.1),
			..default()
		});
	});
}

pub fn inject_attribute(
	commands: &mut Commands,
	entity: Entity,
	value: String,
	font: Handle<Font>,
) {
	commands.entity(entity).with_children(|parent| {
		parent.spawn(Text2dBundle {
			text: Text::from_section(
				value.clone(),
				TextStyle {
					color: Color::from([0., 0., 0., 1.0]),
					font: font.clone(),
					font_size: 64.,
				},
			)
			.with_alignment(TextAlignment::CENTER),
			transform: Transform::from_xyz(0., 0., 0.1),
			..default()
		});

		parent.spawn(Text2dBundle {
			text: Text::from_section(
				value,
				TextStyle {
					color: Color::from([1., 1., 1., 1.0]),
					font,
					font_size: 50.,
				},
			)
			.with_alignment(TextAlignment::CENTER),
			transform: Transform::from_xyz(0., 0., 0.2),
			..default()
		});
	});
}
