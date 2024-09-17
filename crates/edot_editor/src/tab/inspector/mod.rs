use bevy::prelude::*;
use edot_tab::prelude::{CommandsExt, TabBuilder};


pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Inspector>()
            .add_systems(Startup, setup)
        ;
    }
}
#[derive(Debug, Resource)]
pub struct Inspector {
    pub entity:   Entity,
    pub tab_name: Name,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            entity:   Entity::PLACEHOLDER,
            tab_name: Name::from("Inspector"),
        }
    }
}

fn setup(
    mut inspector: ResMut<Inspector>,
    mut commands:  Commands,
) {
    let tab = TabBuilder::new(Name::clone(&inspector.tab_name));

    inspector.entity = commands.register_tab(tab).id();
}