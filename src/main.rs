extern crate sdl2;

mod entityes;
mod utils;

use crate::entityes::player::PlayerState;
use entityes::{fishing::Fishing, player::Player};
use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Texture,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use utils::{Entity, TexturesMap};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Fistude", 160 * 3, 144 * 3)
        .position_centered()
        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let _ = canvas.set_logical_size(160, 144);
    canvas.set_integer_scale(true).unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let renderer = canvas.texture_creator();
    let mut texture_map: HashMap<TexturesMap, Result<Texture<'_>, String>> = HashMap::new();
    texture_map.insert(
        TexturesMap::Background,
        Ok(renderer
            .load_texture("assets/background.bmp")
            .expect("erro")),
    );
    texture_map.insert(
        TexturesMap::Player,
        Ok(renderer.load_texture("assets/player.bmp").expect("erro")),
    );

    let mut player = Player::new(0, 24);
    player.vel = 1;

    let mut fishing = Fishing::new(10, 10);

    canvas.clear();
    canvas.present();

    let mut fps_counter = 0;
    let mut last_fps_update = Instant::now();

    let target_fps: u32 = 30;
    let frame_duration = Duration::new(0, 1_000_000_000 / target_fps);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'game_loop: loop {
        let delta_time_start = std::time::Instant::now();
        canvas.present();
        canvas.clear();

        let texture = texture_map.get(&TexturesMap::Background).unwrap().as_ref();
        canvas
            .copy(
                &texture.unwrap(),
                Rect::new(0, 0, 160, 144),
                Rect::new(0, 0, 160, 144),
            )
            .unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game_loop;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let _ = player.set_state(PlayerState::MoveLeft);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let _ = player.set_state(PlayerState::Default);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let _ = player.set_state(PlayerState::MoveRight);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let _ = player.set_state(PlayerState::Default);
                }
                _ => {}
            }
        }
        fps_counter += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            println!("FPS: {}", fps_counter);
            fps_counter = 0;
            last_fps_update = Instant::now();
        }
        render_entityes!(canvas, texture_map, player, fishing);

        let elapsed_time = delta_time_start.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }
}
