extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;

mod render;
mod update;

use crate::render::render;
use crate::update::{update, spawn_food};
use crate::update::UpdateValue;
use crate::Direction::NORTH;

const MAP_X: u32 = 40;
const MAP_Y: u32 = 40;
const MAP_SIZE: u32 = MAP_X * MAP_Y;
const MAP_TILE_SIZE: u32 = 10;

const MAX_FOOD: u8 = 2;

#[derive(Copy, Clone)]
enum Direction {NORTH, EAST, SOUTH, WEST}

#[derive(Copy, Clone)]
pub struct Pos {
    x: u32,
    y: u32
}

pub struct Snake {
    dir: Direction,
    body: Vec<Pos>
}

pub struct GameData {
    map: [Rect; MAP_SIZE as usize],
    snake: Snake,
    food_positions: [Pos; MAX_FOOD as usize]
}

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
             game_data: &mut GameData) {
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut i = 0;

    loop {
        if let UpdateValue::GameStop = update(&mut event_pump, game_data, &mut i) {
            return
        }
        render(canvas, game_data, i);
        ::std::thread::sleep(Duration::new(0, 4_000_000_000u32 / 60));
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

    let mut game_data = GameData {
        map: init_map(),
        snake: Snake {
            dir: NORTH,
            body: vec![Pos { x: MAP_X / 2, y: MAP_Y / 2 },
                       Pos { x: MAP_X / 2, y: MAP_Y / 2 + 1 },
                       Pos { x: MAP_X / 2, y: MAP_Y / 2 + 2 }]
        },
        food_positions: spawn_food(MAX_FOOD) };

    game_loop(&sdl_context, &mut canvas, &mut game_data);
}
