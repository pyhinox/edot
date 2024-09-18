use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_mod_picking::prelude::{Click, DebugPickingMode, On, Pointer};
use transform_gizmo_bevy::{GizmoTarget, TransformGizmoPlugin};
use edot_editor::EdotEditorPlugin;
use edot_editor::tab::inspector::Inspecting;

fn main() {
    let mut app  = App::new();

    app
        .add_plugins((
            DefaultPlugins,
            EdotEditorPlugin,
            DefaultPickingPlugins,
            TransformGizmoPlugin,
        ))
        .insert_resource(DebugPickingMode::Noisy)
        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Name::new("Inspecting"),
        ColorMesh2dBundle {
            mesh:  meshes.add(Rectangle::new(50., 100.)).into(),
            material: materials.add(Color::WHITE),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::target_insert(
            (
                Inspecting,
                GizmoTarget::default(),
            )
        ),
    ));
    gizmos.circle_2d(Vec2::ZERO, 5.0, Color::WHITE);
}