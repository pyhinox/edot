use bevy::prelude::*;
use bevy_egui::egui::Ui;
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
    let tab = TabBuilder::new(Name::clone(&hierarchy.tab_name))
        .on_show(on_show);
    hierarchy.entity = commands.register_tab(tab).id();
}

fn on_show(_: Entity, world: &mut World, ui: &mut Ui) {
    world
        .query_filtered::<(Entity, Ref<Name>), Without<Parent>>()
        .iter(world)
        .for_each(|(_entity, name)| {
            ui.label(name.as_str());
        });
}