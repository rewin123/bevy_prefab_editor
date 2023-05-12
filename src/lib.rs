use bevy::prelude::*;
use bevy_editor_pls::*;

pub mod panels;


pub struct PrefabEditorPlugin;

impl Plugin for PrefabEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EditorPlugin::default())
            .add_editor_window::<panels::asset_explorer::AssetExplorer>();
    }
}