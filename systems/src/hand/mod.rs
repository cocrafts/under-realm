use bevy::app::StartupStage;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
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
			name: "Bass".to_string(),
			class: ClassType::Assassin,
			kind: CardType::Hero,
			rarity: 6,
		})
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
	mut spine_query: Query<(&mut Spine, Entity, &Card, &Skill)>,
	mut spine_bone_query: Query<(&mut SpineBone, Entity)>,
	mut commands: Commands,
	fonts: Res<Fonts>,
) {
	for event in spine_ready_event.iter() {
		if let Ok((mut spine, _, card, skill)) = spine_query.get_mut(event.entity) {
			let gem_count = card.rarity / 3;
			for level in gem_count..5 {
				let slot_name = format!("s{}", level + 1).to_string();
				if let Some(mut node) = spine.skeleton.find_slot_mut(&slot_name) {
					unsafe {
						node.set_attachment(None);
					}
				}
			}

			for (bone, bone_entity) in spine_bone_query.iter_mut() {
				if let Some(bone) = bone.handle.get(&spine.skeleton) {
					if bone.data().name() == "skill" {
						commands.entity(bone_entity).with_children(|parent| {
							parent.spawn(Text2dBundle {
								text: skill.template.to_text(fonts.vollkorn_bold.clone()),
								transform: Transform::from_xyz(0., 0., 0.1),
								text_2d_bounds: Text2dBounds {
									size: Vec2::new(300.0, 200.0),
								},
								..default()
							});
						});
					}
				}
			}
		}
	}
}
