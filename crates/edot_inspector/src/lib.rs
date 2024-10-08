pub mod root;
pub mod inspectors;
mod primitive;
mod buffer;

use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use crate::primitive::{InspectorPrimitive, InspectorPrimitivePlugin, ReflectInspectorPrimitive};

pub struct EdotInspectorPlugin;

impl Plugin for EdotInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorPrimitivePlugin);
    }
}


pub trait InspectorTypeExt {
    fn register_inspector_type<T>(&mut self) -> &mut Self
    where T: InspectorPrimitive + GetTypeRegistration;
}

impl InspectorTypeExt for SubApp {
    fn register_inspector_type<T>(&mut self) -> &mut Self
    where T: InspectorPrimitive + GetTypeRegistration
    {
        self.register_type::<T>();
        self.register_type_data::<T, ReflectInspectorPrimitive>();
        self
    }
}

impl InspectorTypeExt for App {
    fn register_inspector_type<T>(&mut self) -> &mut Self
    where
        T: InspectorPrimitive + GetTypeRegistration,
    {
        self.main_mut().register_inspector_type::<T>();
        self
    }
}