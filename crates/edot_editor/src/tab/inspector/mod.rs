use bevy::ecs::component::ComponentInfo;
use bevy::prelude::*;
use bevy_egui::egui;
use edot_inspector::inspectors::inspect_entity_component;
use edot_inspector::root::{EntityComponent};
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

#[derive(Default, Debug, Component)]
#[component(storage="SparseSet")]
pub struct Inspecting;

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
    let tab = TabBuilder::new(Name::clone(&inspector.tab_name))
        .on_show(on_show);

    inspector.entity = commands.register_tab(tab).id();
}

fn on_show(_: Entity, world: &mut World, ui: &mut egui::Ui) {
    let targets: Vec<_> = world
        .query_filtered::<Entity, With<Inspecting>>()
        .iter(world).collect();
    targets.into_iter().take(1).for_each(|target| {
        let component_ids: Vec<_> = world
            .inspect_entity(target).into_iter()
            .map(ComponentInfo::id)
            .collect();
        for id in component_ids {
            let target = EntityComponent::new(target, id);
            inspect_entity_component(target, world, ui);
        };
    })
}