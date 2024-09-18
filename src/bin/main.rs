use bevy::prelude::*;
use edot_editor::EdotEditorPlugin;
use edot_editor::tab::inspector::Inspecting;

fn main() {
    let mut app  = App::new();

    app
        .add_plugins((
            DefaultPlugins,
            EdotEditorPlugin,
        ))
        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    commands.spawn((
        TransformBundle::default(),
        Inspecting,
        Name::new("Inspecting")
    ));
    gizmos.circle_2d(Vec2::ZERO, 5.0, Color::WHITE);
}