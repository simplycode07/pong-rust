extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 600;

struct Player {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    speed: i32,
    direction: i8
}

impl Player {
    fn update_pos(&mut self) {
        if self.direction > 0 {
            // Move down
            if self.y + self.speed < (SCREEN_HEIGHT - self.height).try_into().unwrap(){
                self.y += self.speed;
            } else {
                self.y = (SCREEN_HEIGHT - self.height).try_into().unwrap();
                self.direction = 0;
            }
        } else if self.direction == 0 {
            // Move up
            if self.y >= self.speed {
                self.y -= self.speed;
            } else {
                self.y = 0;
                self.direction = 1;
            }
        }
    }
}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("hello", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut running = true;
    let mut player = Player{x:20, y:0, width:10, height:50, speed:10, direction:1};
    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        player.update_pos();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(player.x, player.y, player.width, player.height)).unwrap();

        println!("current player_stats {} {} {}", player.x, player.y, player.direction);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
