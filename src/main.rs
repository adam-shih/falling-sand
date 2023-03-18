use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    window::WindowResizeConstraints,
};
use falling_sand::{
    cellmap::CellMap,
    components::Base,
    constants::SPRITE_SIZE,
    element::Element,
    resources::CursorOnUI,
    systems::*,
    ui::{cursor_on_ui, handle_buttons, setup_ui},
};
use wasm_bindgen::prelude::*;

fn main() {
    // let system_set = SystemSet::new()
    //     .with_system(process_cells)
    //     .with_system(update_transforms)
    //     .with_system(draw_cells)
    //     .with_system(cursor_on_ui)
    //     .with_system(handle_buttons)
    //     .with_system(select_element);

    let system_set = (
        process_cells,
        update_transforms,
        draw_cells,
        cursor_on_ui,
        handle_buttons,
        select_element,
    );

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling Sand".to_string(),
                resolution: (500., 500.).into(),
                resize_constraints: WindowResizeConstraints {
                    max_width: 500.,
                    max_height: 500.,
                    min_width: 500.,
                    min_height: 500.,
                },
                canvas: Some("#bevy-canvas".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_startup_system(populate_cells.in_base_set(StartupSet::PostStartup))
        .add_systems(system_set)
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
    commands.insert_resource(Element::Sand);
    commands.insert_resource(CursorOnUI(false));
}
