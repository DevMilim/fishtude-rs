use sdl2::{render::Canvas, video::Window};

use engine::core::errors::Errors;
use crate::entityes::player::PlayerState;

#[macro_export]
macro_rules! render_entityes {
    ($canvas:expr, $event_pump:expr, $($entity:expr),*) => {
        $(
            let _ = $entity.render(&mut $canvas);
            let _ = $entity.update();
        )*
    };
}

pub trait Entity {
    fn new(x: i32, y: i32) -> Self;
    fn render(&mut self, canvas: &mut Canvas<Window>) -> Result<(), Errors>;
    fn set_state(&mut self, state: PlayerState) -> Result<(), Errors>;
    fn update(&mut self) -> Result<(), Errors>;
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Sprite {
    texture: Option<String>,
    frame: Option<u8>,
}

impl Sprite {
    fn new() -> Self {
        Sprite {
            texture: None,
            frame: None,
        }
    }
}
