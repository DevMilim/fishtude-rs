use std::time::{Duration, Instant};

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

pub struct Fishing {
    point1: i16,
    point2: i16,
    x: i32,
    y: i32,
    timer: Instant,
    size: i16,
}

impl Fishing {
    pub fn new(x: i32, y: i32) -> Self {
        let size = 100;
        Fishing {
            point1: x as i16 + 20,
            point2: x as i16 - 20,
            x,
            y,
            size,
            timer: Instant::now(),
        }
    }

    pub fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        textures: &std::collections::HashMap<
            crate::utils::TexturesMap,
            Result<sdl2::render::Texture<'_>, String>,
        >,
    ) -> Result<(), engine::core::errors::Errors> {
        let _ = canvas.bezier(
            &[self.x as i16, self.point1, self.point2 as i16, self.x as i16],
            &[
                self.y as i16,
                ((self.size as i16) / 4)  + self.y as i16,
                ((self.size as i16) / 4) * 3 + self.y as i16,
                self.y as i16 + self.size,
            ],
            4,
            Color::RGB(255, 255, 255),
        );

        Ok(())
    }
    pub fn update(&mut self) -> Result<(), engine::core::errors::Errors> {
        let elapsed = self.timer.elapsed();
        if elapsed >= Duration::from_millis(1000 / 30) {
            if self.point1 as i32 == self.x + 20 {
                self.point1 = self.x as i16;
                self.point2 = self.x as i16;
            }
            self.timer = Instant::now();
            self.point1 += 1;
            self.point2 -= 1;
        }
        Ok(())
    }
}
