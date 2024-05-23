extern crate sdl2;

use std::f64::consts::PI;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::time::Duration;
use rand::Rng;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 600;

struct Player {
    rect: Rect,
    speed: i32,
}

struct PingPong {
    rect: Rect,
    radius: u32,
    speed: i32,
    // 0 degrees is +x, 90 degrees is +y
    // similarily 180 is -x, and 270 is -y
    angle: i32
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
    // let mut player = Player{x:20, y:(SCREEN_HEIGHT/2).try_into().unwrap(), width:10, height:50, speed:0};
    let mut player = Player{rect:Rect::new(20, (SCREEN_HEIGHT/2).try_into().unwrap(), 10, 80), speed:0};
    let mut computer = Player{rect:Rect::new((SCREEN_WIDTH - 20).try_into().unwrap(), (SCREEN_HEIGHT/2).try_into().unwrap(), 10, 80), speed:0};
    let mut pong = PingPong{rect:Rect::new(200, 150, 20, 20), radius:20, speed:5, angle:11};

    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        if pong.rect.x > (SCREEN_WIDTH/2).try_into().unwrap() {
            if computer.rect.y < pong.rect.y {
                computer.speed = 5;
            }
            if computer.rect.y > pong.rect.y {
                computer.speed = -5;
            }
        }

        running = pong.update_pos();
        player.update_pos();
        computer.update_pos();

        pong.check_collision(&player);
        pong.check_collision(&computer);

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.fill_rect(player.rect).unwrap();
        canvas.fill_rect(computer.rect).unwrap();
        canvas.fill_rect(pong.rect).unwrap();

        println!("current pong_stats {} {} {}", pong.rect.x, pong.rect.y, pong.angle);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::KeyDown { keycode:Some(Keycode::W), .. } => player.speed = -2,
                Event::KeyDown { keycode:Some(Keycode::S), .. } => player.speed = 2,

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
    // change it from 2 vectors to r𝜃
    fn update_pos(&mut self) -> bool {
        //self.rect.x += (self.direction[0]) as i32;
        //self.rect.y += (self.direction[1]) as i32;

        self.rect.x += ((self.speed as f64) * ((self.angle) as f64 * PI / 180.0).cos()) as i32;
        self.rect.y += ((self.speed as f64) * ((-1 * self.angle) as f64 * PI / 180.0).sin()) as i32;
        
        // make sure the ball doesn't exit the screen
        // top collision
        if self.rect.x < 0 {
            self.rect.x = 0;
            println!("You Lose");
            return false;
        }
        
        // bottom collision
        if self.rect.x > (SCREEN_WIDTH - self.radius).try_into().unwrap() {
            self.rect.x = (SCREEN_WIDTH - self.radius).try_into().unwrap();
            println!("You Win");
            return false;
        }

        if self.rect.y > (SCREEN_HEIGHT - self.radius).try_into().unwrap() {
            self.rect.y = (SCREEN_HEIGHT - self.radius).try_into().unwrap();
            self.angle = 360 - self.angle;
        }

        if self.rect.y < 0{
            self.rect.y = 0;
            self.angle = 360 - self.angle;
        }

        return true;
    }

    fn check_collision(&mut self, player: &Player) -> bool {
        if Rect::has_intersection(&self.rect, player.rect) {
            let mut rng = rand::thread_rng();

            //if self.angle > 0 && self.angle <= 90 {
            //    self.angle = rng.gen_range(90..180);
            //} else if self.angle > 90 && self.angle <= 180 {
            //    self.angle = rng.gen_range(0..90);
            //} else if self.angle > 180 && self.angle <= 270 {
            //    self.angle = rng.gen_range(270..360);
            //} else if self.angle > 270 && self.angle <= 360 {
            //    self.angle = rng.gen_range(180..270);
            //}

            if self.angle > 0 && self.angle <= 180 {
                self.angle = 180 - self.angle;
            }

            if self.angle > 180 && self.angle <= 360 {
                self.angle = 540 - self.angle;
            }

            self.angle += rng.gen_range(-10..10);

            println!("collision detected: {}", self.angle);

        }
        return false;
    }

}

impl Player {
    fn update_pos(&mut self) {
        if self.speed > 0 {
            // Move down
            if self.rect.y + self.speed < (SCREEN_HEIGHT - self.rect.height()).try_into().unwrap(){
                self.rect.y += self.speed;
            } else {
                self.rect.y = (SCREEN_HEIGHT - self.rect.height()).try_into().unwrap();
                self.speed = 0;
            }
        } else if self.speed <= 0 {
            // Move up
            if self.rect.y + self.speed >= 0 {
                self.rect.y += self.speed;
            } else {
                self.rect.y = 0;
                self.speed = 0;
            }
        }
    }
}
