extern crate sdl2;

use std::{f64::consts::PI, time::Duration, cmp::min};
use sdl2::{event::Event, keyboard::Keycode, libc::HUGETLB_FLAG_ENCODE_1GB, pixels::Color, rect::Rect};
use rand::Rng;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 600;

struct Player {
    rect: Rect,
    speed: i32,
    color: Color
}

struct PingPong {
    rect: Rect,
    round_x: f64,
    round_y: f64,
    radius: u32,
    speed: i32,
    angle: i32, 
    color: Color
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
    let mut collided_player = false;
    let mut collided_computer = false;
    // let mut player = Player{x:20, y:(SCREEN_HEIGHT/2).try_into().unwrap(), width:10, height:50, speed:0};
    let mut player = Player{rect:Rect::new(20, (SCREEN_HEIGHT/2).try_into().unwrap(), 10, 80), speed:0, color:Color::RGB(0, 0, 0)};
    let mut computer = Player{rect:Rect::new((SCREEN_WIDTH - 20).try_into().unwrap(), (SCREEN_HEIGHT/2).try_into().unwrap(), 10, 80), speed:0, color:Color::RGB(0, 0, 0)};
    let mut pong = PingPong{rect:Rect::new(10, 150, 20, 20), round_x:0.0, round_y:0.0, radius:20, speed:5, angle:2, color:Color::RGB(255, 0, 0)};

    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        if (pong.angle >= 0 && pong.angle < 90) || (pong.angle >= 270 && pong.angle < 360) {
            let half_height = (computer.rect.height()/2) as i32;
            if computer.rect.y + half_height < pong.rect.y {
                computer.speed = min(5, pong.rect.y - computer.rect.y - half_height);
            }
            if computer.rect.y + half_height > pong.rect.y {
                computer.speed = -1 * min(5,  computer.rect.y + half_height - pong.rect.y);
            }
        }

        running = pong.update_pos();
        player.update_pos();
        computer.update_pos();

        collided_player = pong.check_collision(&player, collided_player);
        collided_computer = pong.check_collision(&computer, collided_computer);

        canvas.set_draw_color(player.color);
        canvas.fill_rect(player.rect).unwrap();

        canvas.set_draw_color(computer.color);
        canvas.fill_rect(computer.rect).unwrap();

        canvas.set_draw_color(pong.color);
        canvas.fill_rect(pong.rect).unwrap();

        //println!("current pong_stats {} {} {} {} {}", pong.rect.x, pong.rect.y, pong.angle, pong.round_x, pong.round_y);
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
        // make sure the ball doesn't exit the screen
        if self.rect.x < 0 {
            self.rect.x = 0;
            println!("You Lose");
            return false;
        }
        
        if self.rect.x > (SCREEN_WIDTH - self.radius).try_into().unwrap() {
            self.rect.x = (SCREEN_WIDTH - self.radius).try_into().unwrap();
            println!("You Win");
            return false;
        }

        // bottom collision
        if self.rect.y > (SCREEN_HEIGHT - self.radius).try_into().unwrap() {
            self.rect.y = (SCREEN_HEIGHT - self.radius).try_into().unwrap();
            self.round_x = 0_f64;
            self.round_y = 0_f64;
            self.angle = 360 - self.angle;
        }

        // top collision
        if self.rect.y < 0{
            self.rect.y = 0;
            self.round_x = 0_f64;
            self.round_y = 0_f64;
            self.angle = 360 - self.angle;
        }

        let change_x = (self.speed as f64) * ((self.angle) as f64 * PI / 180.0).cos() + self.round_y;
        self.rect.x += change_x as i32;
        self.round_x = change_x.fract();

        let change_y = (self.speed as f64) * ((-1 * self.angle) as f64 * PI / 180.0).sin() + self.round_y;
        self.rect.y += change_y as i32;
        self.round_y = change_y.fract();

        println!("x: {:.3} y: {:.3} | round_x: {:.5} round_y: {:.5}", change_x, change_y, self.round_x, self.round_y);

        return true;
    }

    fn check_collision(&mut self, player: &Player, prev_state: bool) -> bool {
        if Rect::has_intersection(&self.rect, player.rect) && !prev_state{
            let mut rng = rand::thread_rng();

            if self.angle >= 0 && self.angle < 180 {
                self.angle = 180 - self.angle;
            } else if self.angle >= 180 && self.angle <= 360 {
                self.angle = 540 - self.angle;
            }

            self.angle += rng.gen_range(-30..30);
            if self.angle < 0 { self.angle = 0 }
            self.angle %= 360;
            self.round_x = 0_f64;
            self.round_y = 0_f64;

            //println!("collision detected: {}", self.angle);

            return true;
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
