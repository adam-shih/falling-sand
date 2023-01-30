use bevy::prelude::*;

#[derive(Component)]
pub struct Base;

#[derive(Resource)]
pub struct BrushSize(pub i32);

pub const SPRITE_SIZE: f32 = 4.;
