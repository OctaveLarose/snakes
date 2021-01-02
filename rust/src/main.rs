extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;

mod render;
mod update;

use crate::render::render;
use crate::update::update;
use crate::update::UpdateValue;

const MAP_X: u32 = 50;
const MAP_Y: u32 = 50;
const MAP_SIZE: u32 = MAP_X * MAP_Y;
const MAP_TILE_SIZE: u32 = 10;

fn init_map() -> [Rect; MAP_SIZE as usize] {
    let mut map: [sdl2::rect::Rect; MAP_SIZE as usize] =
        [sdl2::rect::Rect::new(0, 0, MAP_TILE_SIZE, MAP_TILE_SIZE); MAP_SIZE as usize];

    for i in 0..MAP_SIZE {
        map[i as usize].x = (i % MAP_X * MAP_TILE_SIZE) as i32;
        map[i as usize].y = (i / MAP_X * MAP_TILE_SIZE) as i32;
    }

    map
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
