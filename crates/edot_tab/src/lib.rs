mod tab_manager;
mod tab;


pub mod prelude {
    use std::ops::Not;
    use bevy::ecs::system::EntityCommands;
    use bevy::prelude::*;
    use bevy_egui::EguiPlugin;
    pub use super::tab_manager::{TabManager};
    pub use super::tab::*;
    pub trait CommandsExt {
        fn register_tab(&mut self, tab: impl Into<TabBundle>) -> EntityCommands<'_>;
    }

    impl CommandsExt for Commands<'_, '_> {
        fn register_tab(&mut self, tab: impl Into<TabBundle>) -> EntityCommands<'_> {
            self.spawn(tab.into())
        }
    }

    pub struct EdotTabPlugin;

    impl Plugin for EdotTabPlugin {
        fn build(&self, app: &mut App) {
            app.is_plugin_added::<EguiPlugin>().not().then(|| app.add_plugins(EguiPlugin));
            app
                .init_resource::<TabManager>()
                .add_systems(Update, TabManager::show);
        }
    }
}

