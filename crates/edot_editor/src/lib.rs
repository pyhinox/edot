pub mod tab;

use bevy::prelude::*;
use edot_inspector::EdotInspectorPlugin;
use edot_tab::prelude::EdotTabPlugin;
use crate::tab::EditorTabPlugin;

pub struct EdotEditorPlugin;


impl Plugin for EdotEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EdotInspectorPlugin,
                EdotTabPlugin,
                EditorTabPlugin,
            ));
    }
}
