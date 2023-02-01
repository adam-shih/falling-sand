use bevy::prelude::*;

#[derive(Component)]
pub struct Base;

#[derive(Resource)]
pub struct BrushSize(pub i32);

#[derive(Resource)]
pub struct CursorOnUI(pub bool);

pub const SPRITE_SIZE: f32 = 4.;
