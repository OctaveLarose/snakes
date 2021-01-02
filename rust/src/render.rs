use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::{Snake, MAP_X, GameData, Pos};

fn render_map(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
              map: &[Rect; crate::MAP_SIZE as usize],
              i: u8) {
    canvas.set_draw_color(Color::RGB(i, 64, 255 - &i));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in 0..crate::MAP_SIZE {
        let res: std::result::Result<_, _> = canvas.draw_rect(map[i as usize]);
        if res.is_err() {
            print!("Error when drawing map rectangle {}", i);
        }
    }
}

fn draw_snake(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
              map: &[Rect; crate::MAP_SIZE as usize],
              snake: &Snake)
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for pos in &snake.body {
        let res = canvas.fill_rect(map[(pos.y * MAP_X + pos.x) as usize]);
        if res.is_err() {
            print!("Error when drawing snake rectangle at pos {} {}", pos.x, pos.y);
        }
    }
}

fn draw_food(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
             map: &[Rect; crate::MAP_SIZE as usize],
             food_pos: Pos)
{
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let res = canvas.fill_rect(map[(food_pos.y * MAP_X + food_pos.x) as usize]);

    if res.is_err() {
        print!("Error when drawing food rectangle");
    }
}

pub fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
              game_data: &GameData,
              map_blueshift: u8) {
    render_map(canvas, &game_data.map, map_blueshift);
    draw_snake(canvas, &game_data.map, &game_data.snake);
    draw_food(canvas, &game_data.map, game_data.food_position);
    canvas.present();
}