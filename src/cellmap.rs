use bevy::prelude::*;
use rand::{thread_rng, Rng};
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

    pub fn is_air(&self) -> bool {
        if let Element::Air = self.element {
            true
        } else {
            false
        }
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

    pub fn get_next(&self, pos: &IVec2) -> Option<IVec2> {
        let mut rng = thread_rng();
        let coin = rng.gen_bool(0.5);
        let mut left = false;
        let mut right = false;

        if let Some(_) = self
            .cells
            .get(&IVec2::new(pos.x, pos.y - 1))
            .filter(|x| x.is_air())
        {
            return Some(IVec2::new(pos.x, pos.y - 1));
        }

        if let Some(_) = self
            .cells
            .get(&IVec2::new(pos.x - 1, pos.y - 1))
            .filter(|x| x.is_air())
        {
            left = true;
        }

        if let Some(_) = self
            .cells
            .get(&IVec2::new(pos.x + 1, pos.y - 1))
            .filter(|x| x.is_air())
        {
            right = true;
        }

        if !(left || right) {
            return None;
        }
        // else if left && !right {
        //     return Some(IVec2::new(pos.x - 1, pos.y - 1));
        // } else if right && !left {
        //     return Some(IVec2::new(pos.x + 1, pos.y - 1));
        // } else
        if coin {
            return Some(IVec2::new(pos.x - 1, pos.y - 1));
        } else {
            return Some(IVec2::new(pos.x + 1, pos.y - 1));
        }
    }
}
