use crate::element::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub const DOWN: IVec2 = IVec2::new(0, -1);
pub const LEFT: IVec2 = IVec2::new(-1, 0);
pub const RIGHT: IVec2 = IVec2::new(1, 0);
pub const _UP: IVec2 = IVec2::new(0, 1);
pub const DOWN_LEFT: IVec2 = IVec2::new(-1, -1);
pub const DOWN_RIGHT: IVec2 = IVec2::new(1, -1);
pub const _UP_LEFT: IVec2 = IVec2::new(-1, 1);
pub const _UP_RIGHT: IVec2 = IVec2::new(1, 1);

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
