use bevy::prelude::*;
use edot_tab::prelude::{CommandsExt, TabBuilder};


pub struct HierarchyPlugin;

impl Plugin for HierarchyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Hierarchy>()
            .add_systems(Startup, setup)
        ;
    }
}
#[derive(Debug, Resource)]
pub struct Hierarchy {
    pub entity:   Entity,
    pub tab_name: Name,
}

impl Default for Hierarchy {
    fn default() -> Self {
        Self {
            entity:   Entity::PLACEHOLDER,
            tab_name: Name::new("Hierarchy"),
        }
    }
}

fn setup(
    mut commands:  Commands,
    mut hierarchy: ResMut<Hierarchy>,
) {
    let tab = TabBuilder::new(Name::clone(&hierarchy.tab_name));
    hierarchy.entity = commands.register_tab(tab).id();
}
