use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub trait Physics {
    fn next(pos: IVec2, map: &CellMap) -> Option<(IVec2, &Cell)>;
}

#[derive(Clone)]
pub enum Element {
    Air,
    Sand(Sand),
}

#[derive(Clone)]
pub struct Sand;

impl Physics for Sand {
    fn next(pos: IVec2, map: &CellMap) -> Option<(IVec2, &Cell)> {
        let mut rng = thread_rng();
        let coin = rng.gen_bool(0.5);

        let get_air =
            |pos| map.cells.get(&pos).filter(|x| x.is_air()).map(|x| (pos, x));

        let down = get_air(IVec2::new(pos.x, pos.y - 1));
        let left = get_air(IVec2::new(pos.x - 1, pos.y - 1));
        let right = get_air(IVec2::new(pos.x + 1, pos.y - 1));

        if down.is_some() {
            return down;
        }

        if left.is_none() && right.is_none() {
            return None;
        }

        if coin {
            left
        } else {
            right
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Air => Color::NONE,
            Element::Sand(_) => Color::BISQUE,
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
        matches!(self.element, Element::Air)
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

        let val_a = self.cells.get(a).unwrap().clone();
        let val_b = self.cells.get(b).unwrap().clone();

        self.cells.insert(*a, val_b);
        self.cells.insert(*b, val_a);
    }
}
