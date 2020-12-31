extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use crate::UpdateValue::GameStop;

enum UpdateValue { Ok, GameStop }

const MAP_X: u32 = 50;
const MAP_Y: u32 = 50;
const MAP_SIZE: u32 = MAP_X * MAP_Y;
const MAP_TILE_SIZE: u32 = 10;

fn init_map() -> [Rect; MAP_SIZE as usize] {
    let mut map: [sdl2::rect::Rect; MAP_SIZE as usize] = [sdl2::rect::Rect::new(0, 0, 0, 0); MAP_SIZE as usize];

    for i in 0..MAP_SIZE {
        map[i as usize] = sdl2::rect::Rect::new((i % MAP_X * MAP_TILE_SIZE) as i32,
                                                (i / MAP_X * MAP_TILE_SIZE) as i32,
                                                MAP_TILE_SIZE,
                                                MAP_TILE_SIZE);
    }

    map
}

fn update(event_pump: &mut sdl2::EventPump, i: &mut u8) -> UpdateValue {
    *i = (*i + 1) % 255;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return GameStop;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } |
            Event::KeyDown { keycode: Some(Keycode::Down), .. } |
            Event::KeyDown { keycode: Some(Keycode::Left), .. } |
            Event::KeyDown { keycode: Some(Keycode::Right), .. }  => {
                print!("Key pressed\n");
                println!("{:?}", event);
            },
            _ => {}
        }
    }

    UpdateValue::Ok
}

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
          map: &[Rect; MAP_SIZE as usize],
          i: u8) {
    canvas.set_draw_color(Color::RGB(i, 64, 255 - &i));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in 0..MAP_SIZE {
        let res: std::result::Result<_, _> = canvas.draw_rect(map[i as usize]);
        if res.is_err() {
            print!("Error when drawing rectangle {}", i);
        }
    }

    canvas.present();
}

fn game_loop(sdl_context: &sdl2::Sdl,
             canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
             map: &[Rect; MAP_SIZE as usize]) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    loop {
        if let UpdateValue::GameStop = update(&mut event_pump, &mut i) {
            return
        }
        render(canvas, &map, i);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rusty snake", MAP_X * MAP_TILE_SIZE, MAP_Y * MAP_TILE_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let map = init_map();

    game_loop(&sdl_context, &mut canvas, &map);
}
