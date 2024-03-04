use engine::core::errors::Errors;
use sdl2::{image::LoadTexture, rect::Rect, render::Canvas, EventPump};

use crate::utils::{Entity, Position, Sprite};

pub enum PlayerState {
    Default,
    Mechanic,
    Fishing,
    Back,
    MoveLeft,
    MoveRight
}

pub struct Player {
    state: PlayerState,
    pub gold: i32,
    position: Position,
    sprite: Option<Sprite>,
    vel: i32
}

impl Entity for Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            state: PlayerState::Default,
            gold: 14,
            position: Position { x: x, y: y },
            sprite: None,
            vel: 0
        }
    }

    fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), Errors> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("./assets/player.bmp").unwrap();
        canvas
            .copy(&texture, None, Rect::new(self.position.x, self.position.y, 62, 84))
            .unwrap();

        Ok(())
    }
    
    fn set_state(&mut self, state: PlayerState) -> Result<(), Errors> {
        self.state = state;
        Ok(())
    }
    fn update(&mut self) -> Result<(), Errors> {
        match self.state {
            PlayerState::Default => {

            },
            PlayerState::Mechanic => {},
            PlayerState::Fishing => {},
            PlayerState::Back => {},
            PlayerState::MoveLeft => {
                self.position.x -= self.vel;
            },
            PlayerState::MoveRight => {
                self.position.x += self.vel;
            },
        }
        Ok(())
    }
}
