use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use transform_gizmo_bevy::GizmoCamera;
use crate::tab::game_view::GameView;

#[derive(Component, Default, Debug)]
pub struct GameViewCamera;

impl GameViewCamera {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn((
            GameViewCamera,
            GizmoCamera,
            Camera2dBundle {
                camera: Camera {
                    is_active: false,
                    ..default()
                },
                ..default()
            }
        ));
    }

    pub fn set_active(world: &mut World, active: bool) {
        let mut camera = world
            .query_filtered::<Mut<Camera>, With<GameViewCamera>>()
            .single_mut(world);
        camera.is_active = active;
    }

    pub fn sync_viewport(
        game_view:   Res<GameView>,
        windows:     Query<Ref<Window>, With<PrimaryWindow>>,
        mut cameras: Query<Mut<Camera>, With<GameViewCamera>>)
    {
        let Ok(window) = windows.get_single() else {
            return;
        };
        let mut camera = cameras.single_mut();
        if let Some(Rect{ ref min, ref max}) = game_view.clip_rect {
            let scalar_factory = window.resolution.scale_factor();
            let min = min.clone() * scalar_factory;
            let max = max.clone() * scalar_factory;
            let size = UVec2 {
                x: (max.x - min.x) as u32,
                y: (max.y - min.y) as u32,
            };
            let position = UVec2 {
                x: min.x as u32,
                y: min.y as u32,
            };
            if size.x > window.physical_width() || size.y > window.physical_height() {
                return;
            }
            camera.viewport = Some(Viewport {
                physical_position: position,
                physical_size: size,
                ..default()
            });
        }
    }
}