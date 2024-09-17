use bevy::prelude::*;

pub struct GameViewScenePlugin;

impl Plugin for GameViewScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameViewSceneOptions>()
            .add_systems(Update, draw_grid)
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


#[allow(dead_code)]
fn drag_scene(

) {

}