use crate::cellmap::*;
use crate::common::*;
use bevy::input::mouse::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn populate_cells(
    mut commands: Commands,
    mut map: ResMut<CellMap>,
    base: Query<Entity, With<Base>>,
) {
    let mut rng = thread_rng();
    let base = base.single();

    commands.entity(base).with_children(|builder| {
        for x in 0..map.width {
            for y in 0..map.height {
                // let element = if rng.gen_bool(60. / 100.) || y < 30 {
                //     Element::Air
                // } else {
                //     Element::Sand
                // };

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

pub fn process_cells(mut query: Query<&mut Transform>, mut map: ResMut<CellMap>) {
    let keys = map.cells.keys().cloned().collect::<Vec<_>>();

    for pos in keys {
        let cell = map.cells.get(&pos).unwrap();
        match cell.element {
            Element::Sand => {
                let under = IVec2::new(pos.x, pos.y - 1);

                let under_cell = if let Some(v) = map.cells.get(&under) {
                    v
                } else {
                    continue;
                };

                match under_cell.element {
                    Element::Sand => continue,
                    _ => {}
                }
                Vec3::new(SPRITE_SIZE * pos.x as f32, SPRITE_SIZE * pos.y as f32, 0.);

                let mut cell_transform = query.get_component_mut::<Transform>(cell.entity).unwrap();
                cell_transform.translation = Vec3::new(
                    SPRITE_SIZE * under.x as f32,
                    SPRITE_SIZE * under.y as f32,
                    0.,
                );

                let mut under_transform = query
                    .get_component_mut::<Transform>(under_cell.entity)
                    .unwrap();
                under_transform.translation =
                    Vec3::new(SPRITE_SIZE * pos.x as f32, SPRITE_SIZE * pos.y as f32, 0.);

                map.swap(&pos, &under);
            }
            _ => {}
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
        let pos = window.cursor_position().unwrap().floor() / SPRITE_SIZE;

        let mut cell = map
            .cells
            .get_mut(&IVec2::new(pos.x as i32, pos.y as i32))
            .unwrap();
        cell.element = Element::Sand;

        let mut sprite = query.get_component_mut::<Sprite>(cell.entity).unwrap();
        sprite.color = Color::BISQUE;
    }
}
