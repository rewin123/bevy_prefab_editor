use bevy::prelude::*;
use bevy_editor_pls::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_prefab_editor::PrefabEditorPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
