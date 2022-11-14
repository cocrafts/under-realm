use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DeveloperPlugins;

impl PluginGroup for DeveloperPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(WorldInspectorPlugin::new())
			.add(FrameTimeDiagnosticsPlugin)
	}
}
