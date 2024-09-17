pub mod viewport;

use bevy::prelude::*;
use bevy_egui::egui::{emath, Ui};
use edot_tab::prelude::{CommandsExt, TabBuilder};
use crate::tab::game_view::viewport::GameViewCamera;

#[derive(Debug, Default, Component)]
#[component(storage="SparseSet")]
pub struct GameView {
    pub clip_rect: Option<Rect>,
}

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
            .insert(GameView::default())
            .id()
    }
    fn on_show(tab: Entity, world: &mut World, ui: &mut Ui) {
        GameView::update_clip_rect(tab, world, ui);
        ui.label("GameView");
    }
    fn on_open(_view: Entity, world: &mut World) {
        GameViewCamera::set_active(world, true);
    }

    fn on_close(_: Entity, world: &mut World) {
        GameViewCamera::set_active(world, false);
    }

    fn update_clip_rect(tab: Entity, world: &mut World, ui: &Ui) {
        let mut game_view = world.get_mut::<GameView>(tab).unwrap();
        let emath::Rect {min, max} = ui.clip_rect();
        game_view.clip_rect = Some(Rect {
            min: Vec2 {
                x: min.x,
                y: min.y,
            },
            max: Vec2 {
                x: max.x,
                y: max.y,
            }
        });
    }
}
