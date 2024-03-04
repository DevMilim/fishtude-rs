extern crate sdl2;

mod entityes;
mod utils;

use std::time::Duration;
use utils::Entity;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use entityes::player::Player;

use crate::entityes::player::PlayerState;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Fistude", 144*3, 160*3)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut player = Player::new(200, 200);
    player.vel = 5;


    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
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
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    let _ = player.set_state(PlayerState::MoveLeft);
                }
                Event::KeyUp {keycode: Some(Keycode::Left), ..} => {
                    let _ = player.set_state(PlayerState::Default);
                }
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    let _ = player.set_state(PlayerState::MoveRight);
                }
                Event::KeyUp {keycode: Some(Keycode::Right), ..} => {
                    let _ = player.set_state(PlayerState::Default);
                }
                _ => {
                    
                }
            }
        }
        render_entityes!(canvas, event_pump, player);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
