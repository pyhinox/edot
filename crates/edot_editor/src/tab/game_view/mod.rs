pub mod viewport;
mod scene;

use bevy::prelude::*;
use bevy_egui::egui::{emath, Ui};
use edot_tab::prelude::{CommandsExt, TabBuilder};
use crate::tab::game_view::scene::GameViewScenePlugin;
use crate::tab::game_view::viewport::GameViewCamera;

pub struct GameViewPlugin;
impl Plugin for GameViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameView>()
            .add_plugins((
                GameViewScenePlugin,
            ))
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, GameViewCamera::sync_viewport)
        ;
    }
}

#[derive(Debug, Resource)]
pub struct GameView {
    pub entity:    Entity,
    pub tab_name:  Name,
    pub clip_rect: Option<Rect>,
}

impl Default for GameView {
    fn default() -> Self {
        Self {
            entity:    Entity::PLACEHOLDER,
            tab_name:  Name::new("GameView"),
            clip_rect: None,
        }
    }
}

fn setup(
    mut game_view: ResMut<GameView>,
    mut commands:  Commands,
) {
    GameViewCamera::spawn(&mut commands);
    let tab = TabBuilder::new(Name::clone(&game_view.tab_name))
        .clear_background(false)
        .on_open(|_: Entity, world: &mut World| GameViewCamera::set_active(world, true))
        .on_close(|_: Entity, world: &mut World| GameViewCamera::set_active(world, false))
        .on_show(on_show);
    game_view.entity = commands.register_tab(tab).id();
}

fn on_show(tab: Entity, world: &mut World, ui: &mut Ui) {
    update_clip_rect(tab, world, ui);
    ui.label("GameView");
}
fn update_clip_rect(_: Entity, world: &mut World, ui: &Ui) {
    let mut game_view = world.get_resource_mut::<GameView>().unwrap();
    let emath::Rect {min, max} = ui.clip_rect();
    game_view.clip_rect = Some(Rect {
        min: Vec2 { x: min.x, y: min.y },
        max: Vec2 { x: max.x, y: max.y }
    });
}
