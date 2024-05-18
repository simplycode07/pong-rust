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
}

struct PingPong {
    x: i32,
    y: i32,
    radius: u32,
    direction: [f64; 2]
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
    let mut player = Player{x:20, y:(SCREEN_HEIGHT/2).try_into().unwrap(), width:10, height:50, speed:0};
    let mut pong = PingPong{x:50, y:20, radius:20, direction:[2.0, 5.0]};

    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        player.update_pos();
        pong.update_pos();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(player.x, player.y, player.width, player.height)).unwrap();
        canvas.fill_rect(Rect::new(pong.x, pong.y, pong.radius, pong.radius)).unwrap();

        println!("current player_stats {} {} {}", player.x, player.y, player.speed);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::KeyDown { keycode:Some(Keycode::W), .. } => player.speed = -5,
                Event::KeyDown { keycode:Some(Keycode::S), .. } => player.speed = 5,

                Event::KeyUp { keycode:Some(Keycode::W), .. } 
                | Event::KeyUp { keycode:Some(Keycode::S), .. } => player.speed = 0,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}

impl PingPong {
    fn update_pos(&mut self) {
        self.x += (self.direction[0]) as i32;
        self.y += (self.direction[1]) as i32;

        // make sure the ball doesn't exit the screen
        if self.x < 0 {
            self.x = 0;
            self.direction[0] *= -1.0;
        }
        
        if self.x > (SCREEN_WIDTH - self.radius).try_into().unwrap() {
            self.x = (SCREEN_WIDTH - self.radius).try_into().unwrap();
            self.direction[0] *= -1.0;
        }

        if self.y > (SCREEN_HEIGHT - self.radius).try_into().unwrap() {
            self.y = (SCREEN_HEIGHT - self.radius).try_into().unwrap();
            self.direction[1] *= -1.0;
        }

        if self.y < 0{
            self.y = 0;
            self.direction[1] *= -1.0;
        }

    }
}

impl Player {
    fn update_pos(&mut self) {
        if self.speed > 0 {
            // Move down
            if self.y + self.speed < (SCREEN_HEIGHT - self.height).try_into().unwrap(){
                self.y += self.speed;
            } else {
                self.y = (SCREEN_HEIGHT - self.height).try_into().unwrap();
                self.speed = 0;
            }
        } else if self.speed <= 0 {
            // Move up
            if self.y + self.speed >= 0 {
                self.y += self.speed;
            } else {
                self.y = 0;
                self.speed = 0;
            }
        }
    }
}
