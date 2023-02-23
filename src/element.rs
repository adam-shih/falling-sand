use crate::cellmap::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

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
            Element::Water => Color::BLUE,
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
    let can_move = |pos| match map.cells.get(&(pos)) {
        Some(v) => match v.element {
            Element::Air => true,
            _ => false,
        },
        None => false,
    };

    let disperse = |dir: IVec2| {
        let range = 5;
        let mut res = 1;

        for i in 1..range {
            if !can_move(pos + dir * i) {
                break;
            }
            res = i;
        }

        res * dir
    };

    let dest = if can_move(pos + DOWN) {
        Some(DOWN + pos)
    } else if can_move(pos + DOWN_LEFT) && can_move(pos + DOWN_RIGHT) {
        Some(coin_flip(DOWN_LEFT, DOWN_RIGHT) + pos)
    } else if can_move(pos + DOWN_LEFT) {
        Some(DOWN_LEFT + pos)
    } else if can_move(pos + DOWN_RIGHT) {
        Some(DOWN_RIGHT + pos)
    } else if can_move(pos + LEFT) && can_move(pos + RIGHT) {
        Some(disperse(coin_flip(LEFT, RIGHT)) + pos)
    } else if can_move(pos + LEFT) {
        Some(disperse(LEFT) + pos)
    } else if can_move(pos + RIGHT) {
        Some(disperse(RIGHT) + pos)
    } else {
        None
    };

    if dest.is_some() {
        map.swap(&pos, &dest.unwrap());
    }
}
