use std::sync::Arc;
use bevy::prelude::World;
use bevy::reflect::{ParsedPath, ReflectRef};
use bevy_egui::egui;
use crate::root::{EntityComponent, InspectorContext, InspectorRoot};

#[allow(unused_variables)]
#[allow(dead_code)]
fn inspect_entity_component(target: EntityComponent, world: &mut World, ui: &mut egui::Ui) {
    || -> Option<()> {
        let path = ParsedPath(vec![]);
        let target_ref = target.reflect_ref(world, &path)?;
        let cx = InspectorContext {
            root:       Arc::new(target),
            name:       String::from(target_ref.reflect_short_type_path()),
            field_path: path,
            attributes: None,
        };
        match target_ref.reflect_ref() {
            ReflectRef::Struct(val) => {}
            ReflectRef::TupleStruct(_) => unimplemented!(),
            ReflectRef::Tuple(_) => unimplemented!(),
            ReflectRef::List(_) => unimplemented!(),
            ReflectRef::Array(_) => unimplemented!(),
            ReflectRef::Map(_) => unimplemented!(),
            ReflectRef::Enum(_) => unimplemented!(),
            ReflectRef::Value(_) => unimplemented!(),
        }
        Some(())
    }();
}