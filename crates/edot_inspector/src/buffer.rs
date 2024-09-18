use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Resource, Debug)]
pub struct InspectorBuffer {
    pub holder: egui::Id,
    pub buffer: String,
}

impl Default for InspectorBuffer {
    fn default() -> Self {
        Self {
            holder: egui::Id::NULL,
            buffer: String::new(),
        }
    }
}

pub(crate) struct InspectorBufferPlugin;

impl Plugin for InspectorBufferPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InspectorBuffer>()
        ;
    }
}