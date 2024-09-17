use bevy::prelude::*;
use edot_editor::EdotEditorPlugin;

fn main() {
    let mut app  = App::new();

    app
        .add_plugins((
            DefaultPlugins,
            EdotEditorPlugin,
        ))
        .add_systems(Update, setup);

    app.run();
}

fn setup(
    mut _commands: Commands,
    mut gizmos: Gizmos,
) {
    gizmos.circle_2d(Vec2::ZERO, 5.0, Color::WHITE);
}