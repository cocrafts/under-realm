use bevy::app::StartupStage;
use bevy::prelude::*;
use bevy::text::{Text, Text2dBounds, Text2dBundle};
use bevy_spine::{Spine, SpineBone, SpineBundle, SpineReadyEvent};
use components::*;

pub struct HandPlugin;
impl Plugin for HandPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, init)
			.add_system(card_spawned);
	}
}

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
	let te = r#"[
		{ "text": "Passive: ", "color": [0, 0, 0, 1] },
		{ "text": "very", "color": [1, 0, 0, 1] },
		{ "text": " super uttra long skill description that would need more than a row to display" }
	]"#;
	let skill: Vec<TemplateFragment> = serde_json::from_str(te).unwrap();

	commands
		.spawn(SpineBundle {
			skeleton: skeletons.card.clone(),
			transform: Transform::from_xyz(0., 0., 0.1),
			..default()
		})
		.insert(Card {
			id: "secret".to_string(),
			name: "Valkyrie".to_string(),
			class: ClassType::Assassin,
			kind: CardType::Hero,
			rarity: 6,
		})
		.insert(Health(20))
		.insert(Defense(0))
		.insert(Attack(12))
		.insert(Skill {
			template: skill,
			inspire: None,
			activation: Some(ActivationType::Charge),
			charge: Some(3),
		})
		.insert(FromPlayer)
		.insert(Name::new("Card"));
}

pub fn card_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut spine_query: Query<(
		&mut Spine,
		Entity,
		&Card,
		&Skill,
		&Health,
		&Defense,
		&Attack,
	)>,
	mut spine_bone_query: Query<(&mut SpineBone, Entity)>,
	mut commands: Commands,
	fonts: Res<Fonts>,
	textures: Res<GameTextures>,
) {
	for event in spine_ready_event.iter() {
		if let Ok((spine, _, card, skill, health, defense, attack)) =
			spine_query.get_mut(event.entity)
		{
			for (bone, bone_entity) in spine_bone_query.iter_mut() {
				if let Some(bone) = bone.handle.get(&spine.skeleton) {
					let font = fonts.vollkorn.bold.clone();

					if bone.data().name() == "gems" {
						attach_gems(&mut commands, bone_entity, &textures.gem, card.rarity);
					} else if bone.data().name() == "skill" {
						let text = skill.to_text(fonts.fira.clone());
						inject_skill(&mut commands, bone_entity, text);
					} else if bone.data().name() == "name" {
						inject_name(&mut commands, bone_entity, card.name.to_string(), font)
					} else if bone.data().name() == "health" {
						inject_attribute(&mut commands, bone_entity, health.0.to_string(), font);
					} else if bone.data().name() == "defense" {
						inject_attribute(&mut commands, bone_entity, defense.0.to_string(), font);
					} else if bone.data().name() == "attack" {
						inject_attribute(&mut commands, bone_entity, attack.0.to_string(), font);
					}
				}
			}
		}
	}
}

fn attach_gems(commands: &mut Commands, entity: Entity, texture: &Handle<Image>, rarity: u8) {
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

fn inject_skill(commands: &mut Commands, entity: Entity, text: Text) {
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

fn inject_name(
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

fn inject_attribute(commands: &mut Commands, entity: Entity, value: String, font: Handle<Font>) {
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
