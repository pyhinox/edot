use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::emath::Numeric;
use bevy_egui::egui::Ui;
use crate::InspectorTypeExt;
use crate::primitive::InspectorPrimitive;
use crate::root::InspectorContext;

pub struct NumericInspectorPrimitivePlugin;

impl Plugin for NumericInspectorPrimitivePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_inspector_type::<u8>()
            .register_inspector_type::<u16>()
            .register_inspector_type::<u32>()
            .register_inspector_type::<u64>()
            .register_inspector_type::<i8>()
            .register_inspector_type::<i16>()
            .register_inspector_type::<i32>()
            .register_inspector_type::<i64>()
            .register_inspector_type::<f32>()
            .register_inspector_type::<f64>()
        ;
    }
}

impl<T> InspectorPrimitive for T
    where T: Reflect + TypePath + 'static + std::fmt::Display + Numeric + std::str::FromStr,
{
    fn show(cx: &InspectorContext, world: &mut World, ui: &mut Ui) {
        egui::Grid::new(cx.name.as_str()).show(ui, |ui| {
            let id = ui.label(cx.name.as_str()).id;
            let mut display = {
                match cx.get_buffer(world, id) {
                    None => format!("{}", cx.value_ref::<T>(world)),
                    Some(buf) => buf,
                }
            };
            let resp = ui
                .text_edit_singleline(&mut display)
                .labelled_by(id);
            if resp.lost_focus() {
                if let Ok(new_value) = display.parse::<T>() {
                    cx.set_value(world, new_value);
                }
                cx.rel_buffer(world);
            }
            if resp.changed() {
                cx.set_buffer(world, id, display);
            };
        });
    }
}