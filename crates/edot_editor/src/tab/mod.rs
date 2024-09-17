use bevy::prelude::*;
use edot_tab::prelude::TabManager;
use crate::tab::game_view::{GameView, GameViewPlugin};
use crate::tab::hierarchy::Hierarchy;
use crate::tab::inspector::Inspector;

pub mod game_view;
pub mod inspector;
mod hierarchy;

pub(crate) struct EditorTabPlugin;

impl Plugin for EditorTabPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameViewPlugin)
            .add_systems(PostStartup, setup)
        ;
    }
}

fn setup(
    mut commands:    Commands,
    mut tab_manager: ResMut<TabManager>,
    game_view:       Res<GameView>,
) {
    let game_view = game_view.entity;
    let inspector = Inspector::spawn(&mut commands);
    let hierarchy = Hierarchy::spawn(&mut commands);
    let root = tab_manager.set_root_tab(game_view);

    tab_manager.split_right(root, 0.75, vec![inspector]);
    tab_manager.split_left(root, 0.20, vec![hierarchy]);
}