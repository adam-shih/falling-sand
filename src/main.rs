use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod cellmap;
mod common;
mod element;
mod systems;

use bevy::window::WindowResizeConstraints;
use cellmap::*;
use common::*;
use element::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Falling Sand".to_string(),
                width: 500.,
                height: 500.,
                resize_constraints: WindowResizeConstraints {
                    max_width: 500.,
                    max_height: 500.,
                    min_width: 500.,
                    min_height: 500.,
                },
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, populate_cells)
        .add_system(process_cells)
        .add_system(update_transforms)
        .add_system(draw_cells)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    let map = CellMap::new(125, 125);

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
    commands.insert_resource(Element::Water(Water));
}
