use std::collections::HashMap;

use sdl2::{rect::Rect, render::{Canvas, Texture}, video::Window};

use crate::entityes::player::PlayerState;
use engine::core::errors::Errors;

#[macro_export]
macro_rules! render_entityes {
    ($canvas:expr, $texture_map:expr,$($entity:expr),*) => {
        $(
            let _ = $entity.update();
            let _ = $entity.render(&mut $canvas, &$texture_map);
        )*
    };
}

pub trait Entity {
    fn new(x: i32, y: i32) -> Self;
    fn render(&mut self, canvas: &mut Canvas<Window>, textures: &HashMap<TexturesMap, Result<Texture<'_>, String>>) -> Result<(), Errors>;
    fn set_state(&mut self, state: PlayerState) -> Result<(), Errors>;
    fn update(&mut self) -> Result<(), Errors>;
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Sprite {
    pub frames: Option<Vec<Rect>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TexturesMap{
    Background,
    Player
}


pub fn change_sprite(size_x: u32, size_y: u32, amount_x: i32, amount_y: i32, sprite_index: i32) -> Rect{

    Rect::new(0 * sprite_index, 0 * sprite_index, size_x, size_y)
}


