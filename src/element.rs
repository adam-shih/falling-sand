use crate::cellmap::{Direction, *};
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

fn random_choice(
    dir1: Direction,
    dir2: Direction,
    pos: IVec2,
    mut rng: ThreadRng,
) -> Option<IVec2> {
    if rng.gen_bool(0.5) {
        coord(dir1, pos)
    } else {
        coord(dir2, pos)
    }
}

fn handle_sand(pos: IVec2, map: &mut CellMap) {
    let rng = thread_rng();
    let neighbor_elements = map.neighbor_elements(pos);
    let can_move = |dir| match neighbor_elements[dir as usize] {
        Element::Air | Element::Water => true,
        _ => false,
    };

    let dest = if can_move(Direction::Down) {
        coord(Direction::Down, pos)
    } else if can_move(Direction::DownLeft) && can_move(Direction::DownRight) {
        random_choice(Direction::DownLeft, Direction::DownRight, pos, rng)
    } else if can_move(Direction::DownLeft) {
        coord(Direction::DownLeft, pos)
    } else if can_move(Direction::DownRight) {
        coord(Direction::DownRight, pos)
    } else {
        None
    };

    if dest.is_some() {
        map.swap(&pos, &dest.unwrap());
    }
}

fn handle_water(pos: IVec2, map: &mut CellMap) {
    let rng = thread_rng();
    let neighbor_elements = map.neighbor_elements(pos);
    let can_move = |dir| match neighbor_elements[dir as usize] {
        Element::Air => true,
        _ => false,
    };

    let dest = if can_move(Direction::Down) {
        coord(Direction::Down, pos)
    } else if can_move(Direction::DownLeft) && can_move(Direction::DownRight) {
        random_choice(Direction::DownLeft, Direction::DownRight, pos, rng)
    } else if can_move(Direction::DownLeft) {
        coord(Direction::DownLeft, pos)
    } else if can_move(Direction::DownRight) {
        coord(Direction::DownRight, pos)
    } else if can_move(Direction::Left) && can_move(Direction::Right) {
        random_choice(Direction::Left, Direction::Right, pos, rng)
    } else if can_move(Direction::Left) {
        coord(Direction::Left, pos)
    } else if can_move(Direction::Right) {
        coord(Direction::Right, pos)
    } else {
        None
    };

    if dest.is_some() {
        map.swap(&pos, &dest.unwrap());
    }
}
