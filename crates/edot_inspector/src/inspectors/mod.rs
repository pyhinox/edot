use std::sync::Arc;
use bevy::prelude::{AppTypeRegistry, World};
use bevy::reflect::{Access, OffsetAccess, ParsedPath, Reflect, ReflectKind, ReflectRef, TypeInfo};
use bevy_egui::egui;
use bevy_egui::egui::Ui;
use crate::primitive::ReflectInspectorPrimitive;
use crate::root::{EntityComponent, InspectorContext, InspectorRoot};

pub fn inspect_entity_component(target: EntityComponent, world: &mut World, ui: &mut egui::Ui) {
    ui.ctx().options_mut(|options| options.warn_on_id_clash = false);
    let path = ParsedPath(vec![]);
    let Some(target_ref) = target.reflect_ref(world, &path) else {
        return;
    };
    let cx = InspectorContext {
        root:       Arc::new(target),
        name:       String::from(target_ref.reflect_short_type_path()),
        field_path: path,
        attributes: None,
    };
    inspect(&cx, world, ui);
}

pub fn inspect(cx: &InspectorContext, world: &mut World, ui: &mut Ui) {
    let primitive = {
        let registry = world.resource::<AppTypeRegistry>().read();
        let type_id =  {
            let type_info = cx.reflect_ref(world).get_represented_type_info().unwrap();
            type_info.type_id()
        };
        registry
            .get_type_data::<ReflectInspectorPrimitive>(type_id)
            .cloned()
    };
    if let Some(primitive) = primitive {
        primitive.show(cx, world, ui);
        return;
    }
    let show = |ui: &mut Ui| {
        let target_ref = cx.reflect_ref(world);
        match target_ref.reflect_ref() {
            ReflectRef::Struct(_) => inspect_struct(cx, world, ui),
            _ => (),
            // ReflectRef::TupleStruct(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::Tuple(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::List(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::Array(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::Map(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::Enum(val) => println!("{}", val.reflect_type_path()),
            // ReflectRef::Value(val) => println!("{}", val.reflect_type_path()),
        };
    };
    egui::CollapsingHeader::new(cx.name.as_str())
        .default_open(false)
        .show(ui, show);
}

fn inspect_struct(cx: &InspectorContext, world: &mut World, ui: &mut egui::Ui) {
    let ReflectRef::Struct(target) = cx.reflect_ref(world).reflect_ref() else {
        unreachable!();
    };
    let type_info = target.get_represented_type_info().unwrap();
    let field_names = {
        let field_count = target.field_len();
        let mut names = Vec::with_capacity(field_count);
        for i in 0..field_count {
            let field = target.field_at(i).unwrap();
            if !is_option_none(field) {
                let field_name = target.name_at(i).unwrap();
                names.push((i, String::from(field_name)));
            }
        }
        names
    };
    field_names.into_iter().for_each(|(i, name)| {
        let mut field_path = cx.field_path.clone();
        field_path.0.push(OffsetAccess {
            access: Access::FieldIndex(i),
            offset: None,
        });
        let TypeInfo::Struct(struct_info) = type_info else {
            unreachable!();
        };
        let field_info = struct_info.field_at(i).unwrap();
        let attributes = field_info.custom_attributes();
        let cx = InspectorContext {
            root: Arc::clone(&cx.root),
            name,
            field_path,
            attributes: Some(attributes),
        };
        inspect(&cx, world, ui);
    });
}

fn is_option_none(target: &dyn Reflect) -> bool {
    const PREFIX: &str = "core::option::Option";
    if target.reflect_kind() == ReflectKind::Enum && target.reflect_type_path().starts_with(PREFIX) {
        let ReflectRef::Enum(enum_ref) = target.reflect_ref() else {
            unreachable!()
        };
        enum_ref.variant_name() == "None"
    } else {
        false
    }
}