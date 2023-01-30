use crate::cellmap::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub trait Physics {
    fn next(pos: IVec2, map: &CellMap) -> Option<(IVec2, &Cell)>;
}

#[derive(Resource, Clone)]
pub enum Element {
    Air,
    Sand(Sand),
    Water(Water),
    Stone,
}

#[derive(Clone)]
pub struct Water;

impl Physics for Water {
    fn next(pos: IVec2, map: &CellMap) -> Option<(IVec2, &Cell)> {
        None
    }
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
            Element::Water(_) => Color::AQUAMARINE,
            Element::Stone => Color::GRAY,
        }
    }
}
