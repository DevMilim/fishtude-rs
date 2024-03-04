use std::collections::HashMap;

use engine::core::errors::Errors;
use sdl2::{image::LoadTexture, rect::Rect, render::{Canvas, Texture}, EventPump};

use crate::utils::{Entity, Position, Sprite, TexturesMap};

pub enum PlayerState {
    Default,
    Mechanic,
    Fishing,
    Back,
    MoveLeft,
    MoveRight,
}

pub struct Player {
    pub state: PlayerState,
    pub gold: i32,
    pub position: Position,
    pub frames: Option<Vec<Rect>>,
    pub vel: i32,
}

impl Entity for Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            state: PlayerState::Default,
            gold: 14,
            position: Position { x: x, y: y },
            frames: None,
            vel: 0,
        }
    }

    fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>, textures: &HashMap<TexturesMap, Result<Texture<'_>, String>>) -> Result<(), Errors> {
        let texture = textures.get(&TexturesMap::Player).unwrap().as_ref();
        canvas
            .copy(
                &texture.unwrap(),
                None,
                Rect::new(self.position.x, self.position.y, 64, 32)
            )
            .unwrap();
        Ok(())
    }

    fn set_state(&mut self, state: PlayerState) -> Result<(), Errors> {
        self.state = state;
        Ok(())
    }
    fn update(&mut self) -> Result<(), Errors> {
        match self.state {
            PlayerState::Default => {}
            PlayerState::Mechanic => {}
            PlayerState::Fishing => {}
            PlayerState::Back => {}
            PlayerState::MoveLeft => {
                self.position.x -= self.vel;
            }
            PlayerState::MoveRight => {
                self.position.x += self.vel;
            }
        }
        Ok(())
    }
}
