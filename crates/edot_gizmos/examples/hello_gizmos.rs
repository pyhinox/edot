#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::f32::consts::FRAC_PI_2;
use bevy::prelude::*;
use bevy::render::mesh::AnnulusMeshBuilder;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
    ;

    app.run();
}

#[derive(Component, Default, Debug)]
pub struct Target;

fn setup(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh:     meshes.add(AnnulusMeshBuilder::new(40., 30., 64)).into(),
                material: materials.add(Color::srgba(255., 0., 125., 0.699999988)).into(),
                ..default()
            },
            Target,
        )
    );
    commands.spawn(Camera2dBundle::default());
}