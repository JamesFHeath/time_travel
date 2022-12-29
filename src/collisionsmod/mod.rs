use crate::*;
use bevy::app::PluginGroupBuilder;

pub mod collisions;
pub mod components;

pub struct CollisionsModPluginGroup;

impl PluginGroup for CollisionsModPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CollisionsPlugin)
    }
}
