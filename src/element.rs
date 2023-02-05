use crate::cellmap::{Direction::*, *};
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
        neighbor_elements[Down as usize],
        Element::Air | Element::Water
    ) {
        map.swap(&pos, &(NEIGHBOR_COORDS[Down as usize] + pos));
    }

    if coin
        && matches!(
            neighbor_elements[DownLeft as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[DownLeft as usize] + pos));
    } else if !coin
        && matches!(
            neighbor_elements[DownRight as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[DownRight as usize] + pos));
    }
}

fn handle_water(pos: IVec2, map: &mut CellMap) {
    let mut rng = thread_rng();
    let coin = rng.gen_bool(0.5);
    let neighbor_elements = map.neighbor_elements(pos);

    if matches!(
        neighbor_elements[Down as usize],
        Element::Air | Element::Water
    ) {
        map.swap(&pos, &(NEIGHBOR_COORDS[Down as usize] + pos));
    }

    if coin
        && matches!(
            neighbor_elements[DownLeft as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[DownLeft as usize] + pos));
    } else if !coin
        && matches!(
            neighbor_elements[DownRight as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[DownRight as usize] + pos));
    }

    if coin
        && matches!(
            neighbor_elements[Left as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Left as usize] + pos));
    } else if !coin
        && matches!(
            neighbor_elements[Right as usize],
            Element::Air | Element::Water
        )
    {
        map.swap(&pos, &(NEIGHBOR_COORDS[Right as usize] + pos));
    }
}

// #[derive(Clone)]
// pub struct Water;

// impl Water {
//     pub fn next(&self, pos: IVec2, map: &CellMap) -> Option<IVec2> {
//         let mut rng = thread_rng();
//         let coin = rng.gen_bool(0.5);
//         let _dispersion_range = 5;

//         let get_valid =
//             |pos| map.cells.get(&pos).filter(|x| x.is_air()).map(|_| pos);

//         let down = get_valid(IVec2::new(pos.x, pos.y - 1));
//         let down_left = get_valid(IVec2::new(pos.x - 1, pos.y - 1));
//         let down_right = get_valid(IVec2::new(pos.x + 1, pos.y - 1));
//         let left = get_valid(IVec2::new(pos.x - 5, pos.y));
//         let right = get_valid(IVec2::new(pos.x + 5, pos.y));

//         if down.is_some() {
//             return down;
//         }

//         if down_left.is_none()
//             && down_right.is_none()
//             && left.is_none()
//             && right.is_none()
//         {
//             return None;
//         }

//         if down_left.is_none() && down_right.is_none() {
//             if coin {
//                 return left;
//             } else {
//                 return right;
//             }
//         }

//         if coin {
//             down_left
//         } else {
//             down_right
//         }
//     }
// }

// #[derive(Clone)]
// pub struct Sand;

// impl Sand {
//     pub fn next(pos: IVec2, map: &CellMap) -> Option<IVec2> {
//         let mut rng = thread_rng();
//         let coin = rng.gen_bool(0.5);

//         let get_valid = |pos| {
//             map.cells
//                 .get(&pos)
//                 .filter(|x| x.is_air() || x.is_water())
//                 .map(|_| pos)
//         };

//         let down = get_valid(IVec2::new(pos.x, pos.y - 1));
//         let down_left = get_valid(IVec2::new(pos.x - 1, pos.y - 1));
//         let down_right = get_valid(IVec2::new(pos.x + 1, pos.y - 1));

//         if down.is_some() {
//             return down;
//         }

//         if down_left.is_none() && down_right.is_none() {
//             return None;
//         }

//         if coin {
//             down_left
//         } else {
//             down_right
//         }
//     }
// }
