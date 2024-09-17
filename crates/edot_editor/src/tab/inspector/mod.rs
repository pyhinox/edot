use bevy::prelude::*;
use edot_tab::prelude::{CommandsExt, TabBuilder};

#[derive(Debug, Default, Component)]
#[component(storage="SparseSet")]
pub struct Inspector;

impl Inspector {
    const NAME: &'static str = "Inspector";

    pub fn spawn(commands: &mut Commands) -> Entity {
        let tab = TabBuilder::new(Self::NAME);

        commands
            .register_tab(tab)
            .insert(Inspector)
            .id()
    }
}
