mod utils;

use crate::components::*;
use crate::utils::assets::TextureAssets;
use crate::utils::{
	assets::{Fonts, Skeletons},
	attribute::*,
	card::*,
	skill::*,
	state::GameState,
};
use bevy::prelude::*;
use bevy_spine::{Spine, SpineBone, SpineBundle, SpineReadyEvent};
use iyes_loopless::prelude::*;
use utils::*;

pub struct CardPlugin;
impl Plugin for CardPlugin {
	fn build(&self, app: &mut App) {
		app.add_enter_system(GameState::InGame, init)
			.add_system(card_spawned.run_in_state(GameState::InGame));
	}
}

fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
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
	textures: Res<TextureAssets>,
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
