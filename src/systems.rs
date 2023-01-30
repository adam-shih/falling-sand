use crate::cellmap::*;
use crate::common::*;
use crate::element::*;
use bevy::input::mouse::*;
use bevy::prelude::*;

pub fn populate_cells(
    mut commands: Commands,
    mut map: ResMut<CellMap>,
    base: Query<Entity, With<Base>>,
) {
    let base = base.single();

    commands.entity(base).with_children(|builder| {
        for x in 0..map.width {
            for y in 0..map.height {
                let element = Element::Air;

                let new_entity = builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(SPRITE_SIZE)),
                            color: element.color(),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            SPRITE_SIZE * x as f32,
                            SPRITE_SIZE * y as f32,
                            0.,
                        ),
                        ..default()
                    })
                    .id();

                map.cells
                    .insert(IVec2::new(x, y), Cell::new(new_entity, element));
            }
        }
    });
}

pub fn process_cells(
    mut query: Query<&mut Transform>,
    mut map: ResMut<CellMap>,
) {
    let keys = map.cells.keys().cloned().collect::<Vec<_>>();

    for pos in keys {
        let cell = map.cells.get(&pos).unwrap();
        if let Element::Sand(_) = cell.element {
            let (next_pos, next_cell) = match Sand::next(pos, &map) {
                Some(v) => v,
                None => continue,
            };

            let mut cell_transform =
                query.get_component_mut::<Transform>(cell.entity).unwrap();
            cell_transform.translation = Vec3::new(
                SPRITE_SIZE * next_pos.x as f32,
                SPRITE_SIZE * next_pos.y as f32,
                0.,
            );

            let mut next_transform = query
                .get_component_mut::<Transform>(next_cell.entity)
                .unwrap();
            next_transform.translation = Vec3::new(
                SPRITE_SIZE * pos.x as f32,
                SPRITE_SIZE * pos.y as f32,
                0.,
            );

            map.swap(&pos, &next_pos);
        }
    }
}

pub fn draw_sand(
    mut map: ResMut<CellMap>,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Sprite>,
) {
    let window = windows.get_primary().unwrap();

    if mouse_input.pressed(MouseButton::Left) {
        let pos = match window.cursor_position() {
            Some(v) => v.ceil() / SPRITE_SIZE,
            None => return,
        };

        let mut cell =
            match map.cells.get_mut(&IVec2::new(pos.x as i32, pos.y as i32)) {
                Some(v) => v,
                None => return,
            };

        cell.element = Element::Sand(Sand);

        let mut sprite =
            query.get_component_mut::<Sprite>(cell.entity).unwrap();
        sprite.color = cell.element.color();
    }
}
