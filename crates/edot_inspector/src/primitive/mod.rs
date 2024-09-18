mod numeric;

use bevy::app::App;
use bevy::prelude::{Plugin, Reflect, TypePath, World};
use bevy::reflect::FromType;
use bevy_egui::egui::Ui;
use crate::buffer::InspectorBufferPlugin;
use crate::primitive::numeric::NumericInspectorPrimitivePlugin;
use crate::root::InspectorContext;

pub trait InspectorPrimitive: Reflect + TypePath + 'static {
    fn show(cx: &InspectorContext, world: &mut World, ui: &mut Ui);
}

pub struct InspectorPrimitivePlugin;
impl Plugin for InspectorPrimitivePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InspectorBufferPlugin)
            .add_plugins(NumericInspectorPrimitivePlugin)
        ;
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