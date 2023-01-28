use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const SPRITE_SIZE: f32 = 3.;

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
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_sand)
        .run();
}

fn setup(mut commands: Commands) {
    let map = CellMap::default();

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
enum Element {
    Air,
    Sand,
}

impl Element {
    fn color(&self) -> Color {
        match *self {
            Element::Air => Color::NONE,
            Element::Sand => Color::BISQUE,
        }
    }
}

#[derive(Component, Clone)]
struct Cell {
    entity: Entity,
    element: Element,
}

impl Cell {
    fn new(entity: Entity, element: Element) -> Self {
        Self { entity, element }
    }
}

#[derive(Resource)]
struct CellMap {
    height: i32,
    width: i32,
    cells: HashMap<Position, Cell>,
}

fn spawn_sand(mut commands: Commands, mut map: ResMut<CellMap>, base: Query<Entity, With<Base>>) {
    let mut rng = thread_rng();
    let base = base.single();

    commands.entity(base).with_children(|builder| {
        for x in 0..map.width {
            for y in 0..map.height {
                if rng.gen_bool(2. / 3.) {
                    continue;
                }

                let new_entity = builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(SPRITE_SIZE)),
                            color: Element::Sand.color(),
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
                    .insert(Position::new(x, y), Cell::new(new_entity, Element::Sand));
            }
        }
    });
}

impl CellMap {
    fn insert(&mut self, pos: Position, cell: Cell) {
        self.cells.insert(pos, cell);
    }

    fn swap(&mut self, a: Position, b: Position) {
        if !(self.cells.contains_key(&a) && self.cells.contains_key(&b)) {
            println!("Cells not found: {:?} and/or {:?}", a, b);
            return;
        }

        let val_a = self.cells.get(&a).unwrap().clone();
        let val_b = self.cells.get(&b).unwrap().clone();

        self.cells.insert(a, val_b);
        self.cells.insert(b, val_a);
    }
}

impl Default for CellMap {
    fn default() -> Self {
        Self {
            height: 50,
            width: 50,
            cells: default(),
        }
    }
}

#[derive(Component)]
struct Base;
