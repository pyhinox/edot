use bevy::prelude::*;
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
    where T: Reflect + TypePath + 'static + std::fmt::Display,
{
    fn show(cx: &InspectorContext, world: &mut World, ui: &mut Ui) {
        ui.label(cx.name.as_str());
        let num = cx.value_ref::<Self>(world);
        let buf = format!("{}", num);
        ui.text_edit_singleline(&mut buf.as_str());
    }
}