use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Element {
    Air,
    Sand,
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Air => Color::NONE,
            Element::Sand => Color::BISQUE,
        }
    }
}

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
        if !(self.cells.contains_key(&a) && self.cells.contains_key(&b)) {
            println!("Cells not found: {:?} and/or {:?}", a, b);
            return;
        }

        let val_a = self.cells.get(&a).unwrap().clone();
        let val_b = self.cells.get(&b).unwrap().clone();

        self.cells.insert(*a, val_b);
        self.cells.insert(*b, val_a);
    }
}
