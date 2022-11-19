use bevy::app::StartupStage;
use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineReadyEvent};
use components::{Card, CardType, ClassType, FromEnemy, FromPlayer, Skeletons};

pub struct HandPlugin;
impl Plugin for HandPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, init)
			.add_system(player_card_spawned)
			.add_system(enemy_card_spawned);
	}
}

pub fn init(mut commands: Commands, skeletons: Res<Skeletons>) {
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
		.insert(FromPlayer)
		.insert(Name::new("Card"));
}

pub fn player_card_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut card_spines: Query<(&Card, &mut Spine), With<FromPlayer>>,
) {
	for _ in spine_ready_event.iter() {
		for (card, mut spine) in card_spines.iter_mut() {
			let gem_count = card.rarity / 3;
			for level in gem_count..5 {
				let slot_name = format!("s{}", level + 1).to_string();
				if let Some(mut node) = spine.skeleton.find_slot_mut(&slot_name) {
					unsafe {
						node.set_attachment(None);
					}
				}
			}
		}
	}
}

pub fn enemy_card_spawned(
	mut spine_ready_event: EventReader<SpineReadyEvent>,
	mut card_spines: Query<(&Card, &mut Spine), With<FromEnemy>>,
) {
	for _ in spine_ready_event.iter() {
		for (card, mut spine) in card_spines.iter_mut() {
			let gem_count = card.rarity / 3;
			for level in gem_count..5 {
				let slot_name = format!("s{}", level + 1).to_string();
				if let Some(mut node) = spine.skeleton.find_slot_mut(&slot_name) {
					unsafe {
						node.set_attachment(None);
					}
				}
			}
		}
	}
}
