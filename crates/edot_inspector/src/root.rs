use std::sync::Arc;
use bevy::ecs::component::ComponentId;
use bevy::prelude::*;
use bevy::reflect::{ParsedPath, Reflect, ReflectFromPtr, ReflectPathError};
use bevy::reflect::attributes::CustomAttributes;
use bevy_egui::egui;
use crate::buffer::InspectorBuffer;

pub enum InspectorRootId {
    Entity(Entity),
}

pub trait InspectorRoot {
    fn id(&self) -> InspectorRootId;
    fn reflect_ref<'a>(&self, world: &'a World, path: &ParsedPath) -> Option<&'a dyn Reflect>;
    fn set_reflect(&self, world: &mut World, path: &ParsedPath, value: &dyn Reflect);
}

pub struct EntityComponent {
    pub entity:       Entity,
    pub component_id: ComponentId
}

impl EntityComponent {
    pub fn new(entity: Entity, component_id: ComponentId) -> Self {
        Self { entity, component_id }
    }

    fn get_from_ptr(&self, world: &World) -> Option<ReflectFromPtr> {
        let comp_type = world.components().get_info(self.component_id)?.type_id()?;
        let registry = world.resource::<AppTypeRegistry>().read();
        Some(
            registry
            .get_type_data::<ReflectFromPtr>(comp_type)?
            .clone()
        )
    }
}
impl InspectorRoot for EntityComponent {
    fn id(&self) -> InspectorRootId {
        InspectorRootId::Entity(self.entity)
    }

    fn reflect_ref<'a>(&self, world: &'a World, path: &ParsedPath) -> Option<&'a dyn Reflect> {
        let from_ptr = self.get_from_ptr(world)?;
        let ptr = world.entity(self.entity).get_by_id(self.component_id)?;
        let comp_ref = unsafe {
            from_ptr.as_reflect(ptr)
        };
        match comp_ref.reflect_path(path) {
            Ok(val) => Some(val),
            Err(ReflectPathError::InvalidAccess(_)) => None,
            Err(err) => panic!("{:?}", err),
        }
    }

    fn set_reflect(&self, world: &mut World, path: &ParsedPath, value: &dyn Reflect) {
        let Some(from_ptr) = self.get_from_ptr(world) else {
            return;
        };
        let mut entity_mut = world.entity_mut(self.entity);
        let Some(mut ptr) = entity_mut.get_mut_by_id(self.component_id) else {
            return;
        };
        let comp_mut = unsafe {
            from_ptr.as_reflect_mut(ptr.as_mut())
        };
        match comp_mut.reflect_path_mut(path) {
            Ok(val_mut) => val_mut.apply(value),
            Err(ReflectPathError::InvalidAccess(_)) => (),
            Err(err) => panic!("{:?}", err),
        };
    }
}

pub struct InspectorContext {
    pub root:        Arc<dyn InspectorRoot>,
    pub name:        String,
    pub field_path:  ParsedPath,
    pub attributes:  Option<&'static CustomAttributes>,
}

impl InspectorContext {
    pub fn reflect_ref<'a>(&self, world: &'a World) -> &'a dyn Reflect {
        self.root.reflect_ref(world, &self.field_path).unwrap()
    }

    pub fn value_ref<'a, T: Reflect>(&self, world: &'a World) -> &'a T {
        let reflect = self.reflect_ref(world);
        reflect.downcast_ref::<T>().unwrap()
    }

    pub fn set_reflect(&self, world: &mut World, value: &dyn Reflect) {
        self.root.set_reflect(world, &self.field_path, value);
    }

    pub fn set_value<T: Reflect>(&self, world: &mut World, value: T) {
        self.set_reflect(world, &value);
    }

    pub fn get_buffer(&self, world: &World, id: egui::Id) -> Option<String> {
        let buf = world.resource::<InspectorBuffer>();
        if buf.holder != id {
            None
        } else {
            Some(buf.buffer.clone())
        }
    }

    pub fn set_buffer(&self, world: &mut World, id: egui::Id, buf: String) {
        *world.resource_mut::<InspectorBuffer>() = InspectorBuffer {
            holder: id,
            buffer: buf,
        };
    }

    pub fn rel_buffer(&self, world: &mut World) {
        self.set_buffer(world, egui::Id::NULL, String::new());
    }
}
