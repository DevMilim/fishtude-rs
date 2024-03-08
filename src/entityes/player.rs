use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use engine::core::errors::Errors;
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
};

use crate::utils::TexturesMap;

use super::fishing::Fishing;

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
    pub x: i32,
    pub y: i32,
    pub vel: i32,
    flip: bool,
    frames: Option<Vec<Rect>>,
    frame_index: u8,
    timer: Instant,
    rod: Option<Fishing>,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            state: PlayerState::Default,
            x,
            y,
            frames: Some(vec![
                Rect::new(0, 0, 16, 16),
                Rect::new(16, 0, 16, 16),
                Rect::new(32, 0, 16, 16),
                Rect::new(48, 0, 16, 16),
                Rect::new(0, 16, 16, 16),
                Rect::new(16, 16, 16, 16),
            ]),
            frame_index: 0,
            vel: 0,
            flip: false,
            timer: Instant::now(),
            rod: None,
        }
    }

    pub fn render(
        &mut self,
        canvas: &mut Canvas<sdl2::video::Window>,
        textures: &HashMap<TexturesMap, Result<Texture<'_>, String>>,
    ) -> Result<(), Errors> {
        let texture = textures.get(&TexturesMap::Player).unwrap().as_ref();
        let src_dist = [Rect::new(0, 0, 16, 16), Rect::new(self.x, self.y, 16, 16)];
        canvas
            .copy_ex(
                &texture.unwrap(),
                <Option<Vec<Rect>> as Clone>::clone(&self.frames).unwrap()
                    [self.frame_index as usize],
                src_dist[1],
                0.0,
                None,
                self.flip,
                false,
            )
            .unwrap();
        Ok(())
    }

    pub fn set_state(&mut self, state: PlayerState) -> Result<(), Errors> {
        self.state = state;
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), Errors> {
        let animation_framerate = (1000.0 / 4.0) as u64;
        match self.state {
            PlayerState::Fishing => {}
            PlayerState::Default => {
                let elapsed = self.timer.elapsed();
                if elapsed >= Duration::from_millis(1000 / 8) {
                    self.frame_index += 1;
                    self.timer = Instant::now();
                    if self.frame_index == 5 {
                        self.frame_index = 0;
                    }
                }
            }
            PlayerState::Mechanic => {}
            PlayerState::Back => {}
            PlayerState::MoveLeft => {
                if self.frame_index <= 3 {
                    self.frame_index = 4;
                }
                self.x -= self.vel;
                self.flip = true;
                let elapsed = self.timer.elapsed();
                if elapsed >= Duration::from_millis(animation_framerate) {
                    self.frame_index += 1;
                    self.timer = Instant::now();
                }
                if self.frame_index == 6 {
                    self.frame_index = 4;
                }
            }
            PlayerState::MoveRight => {
                if self.frame_index <= 3 {
                    self.frame_index = 4;
                }
                self.x += self.vel;
                self.flip = false;
                let elapsed = self.timer.elapsed();
                if elapsed >= Duration::from_millis(animation_framerate) {
                    self.frame_index += 1;
                    self.timer = Instant::now();
                }
                if self.frame_index == 6 {
                    self.frame_index = 4;
                }
            }
        }
        if self.frame_index == 6 {
            self.frame_index = 0;
        }
        Ok(())
    }
}
