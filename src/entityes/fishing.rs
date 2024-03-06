use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use crate::utils::Entity;

pub struct Fishing {}

impl Entity for Fishing {
    fn new(x: i32, y: i32) -> Self {
        Fishing {}
    }

    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        textures: &std::collections::HashMap<
            crate::utils::TexturesMap,
            Result<sdl2::render::Texture<'_>, String>,
        >,
    ) -> Result<(), engine::core::errors::Errors> {
        let _ = canvas.bezier(&[0, 20, 16, 80], &[0, 10, 40, 90], 500, Color::RGB(0, 0, 0));
        Ok(())
    }

    fn set_state(
        &mut self,
        state: super::player::PlayerState,
    ) -> Result<(), engine::core::errors::Errors> {
        Ok(())
    }

    fn update(&mut self) -> Result<(), engine::core::errors::Errors> {
        Ok(())
    }
}
