use crate::element::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
}

// pub const NEIGHBOR_COORDS: HashMap<Direction, IVec2> = [
//     (Down, IVec2::new(0, -1)),
//     (Left, IVec2::new(-1, 0)),
//     (Right, IVec2::new(1, 0)),
//     (Up, IVec2::new(0, 1)),
//     (DownLeft, IVec2::new(-1, -1)),
//     (DownRight, IVec2::new(1, -1)),
//     (UpLeft, IVec2::new(-1, 1)),
//     (UpRight, IVec2::new(1, 1)),
// ]
// .iter()
// .cloned()
// .collect();
pub const NEIGHBOR_COORDS: [IVec2; 8] = [
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, -1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(1, 1),
];

#[derive(Clone)]
pub struct Cell {
    pub entity: Entity,
    pub element: Element,
}

impl Cell {
    pub fn new(entity: Entity, element: Element) -> Self {
        Self { entity, element }
    }
}

#[derive(Resource)]
pub struct CellMap {
    pub height: i32,
    pub width: i32,
    pub cells: HashMap<IVec2, Cell>,
}

impl CellMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: default(),
        }
    }

    pub fn neighbor_elements(&self, pos: IVec2) -> [Element; 8] {
        let mut res = [Element::None; 8];

        let neighbor_coords = NEIGHBOR_COORDS.iter().map(|v| pos + *v);

        for (i, v) in neighbor_coords.enumerate() {
            if let Some(cell) = self.cells.get(&v) {
                res[i] = cell.element;
            }
        }

        res
    }

    pub fn swap(&mut self, a: &IVec2, b: &IVec2) {
        if !(self.cells.contains_key(a) && self.cells.contains_key(b)) {
            println!("Cells not found: {:?} and/or {:?}", a, b);
            return;
        }

        if a == b {
            return;
        }

        let val_a = self.cells.get(a).unwrap().clone();
        let val_b = self.cells.get(b).unwrap().clone();

        self.cells.insert(*a, val_b);
        self.cells.insert(*b, val_a);
    }
}
