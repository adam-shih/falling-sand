use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod cellmap;
mod common;
mod systems;

use cellmap::*;
use common::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 500.,
                height: 500.,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, populate_cells)
        .add_system(process_cells)
        .add_system(update_sprites)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    let map = CellMap::new(50, 50);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            -(map.width as f32 * SPRITE_SIZE) / 2.,
            -(map.height as f32 * SPRITE_SIZE) / 2.,
            0.,
        )),
        Base,
    ));
    commands.insert_resource(map);
}
