use bevy::prelude::*;
use bevy::reflect::{ParsedPath};
use edot_inspector::root::{EntityComponent, InspectorRoot};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(
            (
                MinimalPlugins,
                TransformPlugin,
            )
        )
        .add_systems(
            Update,
            (
                setup,
                apply_deferred,
                test,
            ).chain(),
        )
    ;

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TransformBundle::default());
}
fn test(world: &mut World) {
    for entity in world.query_filtered::<Entity, With<Transform>>().iter(world) {
        let component_id = world.component_id::<Transform>().unwrap();
        let inspector_target = EntityComponent::new(entity, component_id);
        dbg!(&inspector_target.name(world));

        let Some(translation) = inspector_target.reflect_ref(world, &ParsedPath::parse("translation").unwrap()) else {
            continue;
        };
        dbg!(&translation.reflect_type_path());
    }
    world.send_event(AppExit::Success);
}