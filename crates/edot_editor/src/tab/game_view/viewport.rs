use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct GameViewCamera;

impl GameViewCamera {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn((
            GameViewCamera,
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
}