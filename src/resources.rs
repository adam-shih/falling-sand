use crate::element::Element;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BrushSize(pub i32);

#[derive(Resource)]
pub struct CursorOnUI(pub bool);

#[derive(Resource)]
pub struct CurrentElement(pub Element);
