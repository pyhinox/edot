use bevy::prelude::{Reflect, TypePath, World};
use bevy::reflect::FromType;
use bevy_egui::egui::Ui;
use crate::root::InspectorContext;

pub trait InspectorPrimitive: Reflect + TypePath + 'static {
    fn show(cx: &InspectorContext, world: &mut World, ui: &mut Ui);
}

impl InspectorPrimitive for bool {
    fn show(cx: &InspectorContext, _world: &mut World, ui: &mut Ui){
        let mut val = false;
        ui.toggle_value(&mut val, cx.name.as_str());
    }
}

#[derive(Clone)]
pub struct ReflectInspectorPrimitive {
    func: fn (cx: &InspectorContext, world: &mut World, ui: &mut Ui)
}

impl<T> FromType<T> for ReflectInspectorPrimitive
where T: InspectorPrimitive
{
    fn from_type() -> Self {
        Self {
            func: |cx: &InspectorContext, world: &mut World, ui: &mut Ui| {
                T::show(cx, world, ui);
            }
        }
    }
}

impl ReflectInspectorPrimitive {
    pub fn show(self, cx: &InspectorContext, world: &mut World, ui: &mut Ui) {
        (self.func)(cx, world, ui);
    }
}