use crate::cellmap::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[allow(dead_code)]
#[derive(Resource, Clone, Copy)]
pub enum Element {
    None,
    Air,
    Sand,
    Water,
    Stone,
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Air | Element::None => Color::NONE,
            Element::Sand => Color::BISQUE,
            Element::Water => Color::AQUAMARINE,
            Element::Stone => Color::GRAY,
        }
    }

    pub fn next(&self, pos: IVec2, map: &mut CellMap) -> Option<IVec2> {
        match *self {
            Element::Sand => handle_sand(pos, map),
            Element::Water => handle_water(pos, map),
            _ => {}
        }

        None
    }
}

fn coin_flip<T>(heads: T, tails: T) -> T {
    let mut rng = thread_rng();
    if rng.gen_bool(0.5) {
        heads
    } else {
        tails
    }
}

fn handle_sand(pos: IVec2, map: &mut CellMap) {
    let can_move = |dir| match map.cells.get(&(dir + pos)) {
        Some(v) => match v.element {
            Element::Air | Element::Water => true,
            _ => false,
        },
        None => false,
    };

    let dest = if can_move(DOWN) {
        Some(DOWN + pos)
    } else if can_move(DOWN_LEFT) && can_move(DOWN_RIGHT) {
        Some(coin_flip(DOWN_LEFT, DOWN_RIGHT) + pos)
    } else if can_move(DOWN_LEFT) {
        Some(DOWN_LEFT + pos)
    } else if can_move(DOWN_RIGHT) {
        Some(DOWN_RIGHT + pos)
    } else {
        None
    };

    if dest.is_some() {
        map.swap(&pos, &dest.unwrap());
    }
}

fn handle_water(pos: IVec2, map: &mut CellMap) {
    let can_move = |dir| match map.cells.get(&(dir + pos)) {
        Some(v) => match v.element {
            Element::Air => true,
            _ => false,
        },
        None => false,
    };

    let dest = if can_move(DOWN) {
        Some(DOWN + pos)
    } else if can_move(DOWN_LEFT) && can_move(DOWN_RIGHT) {
        Some(coin_flip(DOWN_LEFT, DOWN_RIGHT) + pos)
    } else if can_move(DOWN_LEFT) {
        Some(DOWN_LEFT + pos)
    } else if can_move(DOWN_RIGHT) {
        Some(DOWN_RIGHT + pos)
    } else if can_move(LEFT) && can_move(RIGHT) {
        Some(coin_flip(LEFT, RIGHT) + pos)
    } else if can_move(LEFT) {
        Some(LEFT + pos)
    } else if can_move(RIGHT) {
        Some(RIGHT + pos)
    } else {
        None
    };

    if dest.is_some() {
        map.swap(&pos, &dest.unwrap());
    }
}
