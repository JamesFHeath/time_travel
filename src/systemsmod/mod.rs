use crate::*;
use bevy::app::PluginGroupBuilder;

pub mod general_systems;

pub struct SystemsModPluginGroup;

impl PluginGroup for SystemsModPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GeneralSystemsPlugin)
    }
}
