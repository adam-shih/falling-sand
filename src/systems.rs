use crate::cellmap::*;
use crate::common::*;
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
                let element = if rng.gen_bool(60. / 100.) || y < 30 {
                    Element::Air
                } else {
                    Element::Sand
                };

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
                    .insert(Position::new(x, y), Cell::new(new_entity, element));
            }
        }
    });
}

pub fn process_cells(mut map: ResMut<CellMap>) {
    let keys = map.cells.keys().cloned().collect::<Vec<_>>();

    for pos in keys {
        let cell = map.cells.get(&pos).unwrap();
        match cell.element {
            Element::Sand => {
                let under_cell = if let Some(v) = map.cells.get(&pos.under()) {
                    v
                } else {
                    continue;
                };

                match under_cell.element {
                    Element::Sand => continue,
                    _ => {}
                }

                map.swap(&pos, &pos.under());
            }
            _ => {}
        }
    }
}

pub fn update_sprites(mut query: Query<&mut Transform>, map: Res<CellMap>) {
    for (pos, cell) in map.cells.iter() {
        let mut transform = query.get_component_mut::<Transform>(cell.entity).unwrap();
        transform.translation =
            Vec3::new(SPRITE_SIZE * pos.x as f32, SPRITE_SIZE * pos.y as f32, 0.);
    }
}
