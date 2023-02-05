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

pub fn update_transforms(mut query: Query<&mut Transform>, map: Res<CellMap>) {
    for (pos, cell) in map.cells.iter() {
        let mut transform =
            query.get_component_mut::<Transform>(cell.entity).unwrap();

        transform.translation = Vec3::new(
            SPRITE_SIZE * pos.x as f32,
            SPRITE_SIZE * pos.y as f32,
            0.,
        );
    }
}

pub fn process_cells(mut map: ResMut<CellMap>) {
    let keys = map.cells.keys().cloned().collect::<Vec<_>>();

    for pos in keys {
        let cell = map.cells.get(&pos).unwrap().clone();
        cell.element.next(pos, &mut map);

        // let next = match &cell.element {
        //     Element::Sand(_) => match Sand::next(pos, &map) {
        //         Some(v) => v,
        //         None => continue,
        //     },
        //     Element::Water(water) => match water.next(pos, &map) {
        //         Some(v) => v,
        //         None => continue,
        //     },
        //     _ => continue,
        // };

        // map.swap(&pos, &next);
    }
}

pub fn draw_cells(
    element: Res<Element>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    ptr_on_ui: Res<CursorOnUI>,
    mut map: ResMut<CellMap>,
    mut query: Query<&mut Sprite>,
) {
    if ptr_on_ui.0 {
        return;
    }

    let window = windows.get_primary().unwrap();

    if mouse_input.pressed(MouseButton::Left) {
        let pos = match window.cursor_position() {
            Some(v) => (v.ceil() / SPRITE_SIZE).as_ivec2(),
            None => return,
        };

        let positions = [
            pos,
            pos + IVec2::Y,
            pos + IVec2::ONE,
            pos + IVec2::X,
            pos + IVec2::X + IVec2::NEG_Y,
            pos + IVec2::NEG_Y,
            pos + IVec2::NEG_ONE,
            pos + IVec2::NEG_X,
            pos + IVec2::NEG_X + IVec2::Y,
        ];

        for pos in positions {
            let cell = map.cells.get_mut(&pos);
            match cell {
                Some(c) => {
                    if matches!(c.element, Element::Air) {
                        c.element = element.clone();
                        let mut sprite = query
                            .get_component_mut::<Sprite>(c.entity)
                            .unwrap();
                        sprite.color = c.element.color();
                    }
                }
                None => continue,
            }
        }
    }
}

pub fn cursor_on_ui(
    mut cursor_on_ui: ResMut<CursorOnUI>,
    query: Query<&Interaction>,
) {
    for i in query.iter() {
        if let Interaction::Clicked | Interaction::Hovered = i {
            cursor_on_ui.0 = true;
            return;
        }
    }

    cursor_on_ui.0 = false;
}

pub fn select_element(mut element: ResMut<Element>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        *element = Element::Sand;
    } else if keys.just_pressed(KeyCode::Key2) {
        *element = Element::Water;
    } else if keys.just_pressed(KeyCode::Key3) {
        *element = Element::Stone;
    }
}
