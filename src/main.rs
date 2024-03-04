extern crate sdl2;

mod entityes;
mod utils;

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::Texture};
use std::{collections::HashMap, time::{Duration, Instant}};
use utils::{Entity, TexturesMap};
use entityes::player::Player;
use crate::entityes::player::PlayerState;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Fistude", 160*3, 144*3)
        .position_centered()
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
    texture_map.insert(TexturesMap::Player, Ok(renderer.load_texture("assets/player.bmp").expect("erro")));

    let mut player = Player::new(200, 200);
    player.vel = 1;

    canvas.clear();
    canvas.present();

    let mut fps_counter = 0;
    let mut last_fps_update = Instant::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        canvas.present();
        canvas.clear();
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
        render_entityes!(canvas, texture_map, player);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
