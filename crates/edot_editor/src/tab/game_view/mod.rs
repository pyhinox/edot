mod viewport;

use bevy::prelude::*;
use bevy_egui::egui::Ui;
use edot_tab::prelude::{CommandsExt, TabBuilder};
use crate::tab::game_view::viewport::GameViewCamera;

#[derive(Debug, Default, Component)]
#[component(storage="SparseSet")]
pub struct GameView;

impl GameView {
    const NAME: &'static str = "GameView";
    pub fn spawn(commands: &mut Commands) -> Entity {
        GameViewCamera::spawn(commands);
        let tab = TabBuilder::new(Self::NAME)
            .clear_background(false)
            .on_open(Self::on_open)
            .on_close(Self::on_close)
            .on_show(Self::on_show);
        commands
            .register_tab(tab)
            .insert(GameView)
            .id()
    }
    fn on_open(_view: Entity, world: &mut World) {
        GameViewCamera::set_active(world, true);
    }

    fn on_close(_: Entity, world: &mut World) {
        GameViewCamera::set_active(world, false);
    }

    fn on_show(_view: Entity, _world: &mut World, ui: &mut Ui) {
        ui.label("GameView");
    }
}
