use crate::cellmap::{Direction, *};
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

fn handle_sand(pos: IVec2, map: &mut CellMap) {
    let mut rng = thread_rng();
    let coin = rng.gen_bool(0.5);
    let neighbor_elements = map.neighbor_elements(pos);

    if matches!(
        neighbor_elements[Direction::Down as usize],
        Element::Air | Element::Water
    ) {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::Down as usize] + pos));
    } else if coin
        && matches!(
            neighbor_elements[Direction::DownLeft as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::DownLeft as usize] + pos));
    } else if !coin
        && matches!(
            neighbor_elements[Direction::DownRight as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(
            &pos,
            &(NEIGHBOR_COORDS[Direction::DownRight as usize] + pos),
        );
    }
}

fn handle_water(pos: IVec2, map: &mut CellMap) {
    let mut rng = thread_rng();
    let coin = rng.gen_bool(0.5);
    let neighbor_elements = map.neighbor_elements(pos);

    if matches!(neighbor_elements[Direction::Down as usize], Element::Air) {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::Down as usize] + pos));
    } else if coin
        && matches!(
            neighbor_elements[Direction::DownLeft as usize],
            Element::Air
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::DownLeft as usize] + pos));
    } else if !coin
        && matches!(
            neighbor_elements[Direction::DownRight as usize],
            Element::Air
        )
    {
        map.swap(
            &pos,
            &(NEIGHBOR_COORDS[Direction::DownRight as usize] + pos),
        );
    } else if coin
        && matches!(neighbor_elements[Direction::Left as usize], Element::Air)
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::Left as usize] + pos));
    } else if !coin
        && matches!(neighbor_elements[Direction::Right as usize], Element::Air)
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Direction::Right as usize] + pos));
    }
}
