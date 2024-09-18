use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::tab::game_view::GameView;

pub struct GameViewScenePlugin;

impl Plugin for GameViewScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameViewSceneOptions>()
            .add_systems(
                Update,
                (
                    draw_grid,
                    drag_scene,
                )
            )
        ;
    }
}

#[derive(Debug, Resource)]
pub struct GameViewSceneOptions {
    pub enable_grid:  bool,
    pub grid_spacing: f32,
}

impl Default for GameViewSceneOptions {
    fn default() -> Self {
        Self {
            enable_grid:  true,
            grid_spacing: 25.,
        }
    }
}

fn draw_grid(
    options:    Res<GameViewSceneOptions>,
    mut gizmos: Gizmos,
) {
    if options.enable_grid {
        gizmos.grid_2d(
            Vec2::ZERO,
            0.,
            UVec2::splat(50),
            Vec2::splat(options.grid_spacing),
            LinearRgba::gray(0.05)
        );
    }
}


fn drag_scene(
    windows:                 Query<Ref<Window>, With<PrimaryWindow>>,
    game_view:               Res<GameView>,
    mut mouse_motion:        Local<Vec2>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button:            Res<ButtonInput<MouseButton>>,
) {
    let Some(mouse_position) = windows
        .get_single().ok()
        .and_then(|window| window.cursor_position())
        .zip(game_view.clip_rect)
        .and_then(|(mouse_position, clip_rect)|
            clip_rect.contains(mouse_position).then_some(mouse_position)
        )
    else {
        return;
    };

    *mouse_motion = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        *mouse_motion += event.delta;
    }
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    dbg!(&mouse_position);
}