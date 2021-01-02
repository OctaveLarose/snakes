use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::{Snake, MAP_X, GameData};

fn render_map(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
              map: &[Rect; crate::MAP_SIZE as usize],
              i: u8) {
    canvas.set_draw_color(Color::RGB(i, 64, 255 - &i));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in 0..crate::MAP_SIZE {
        let res: std::result::Result<_, _> = canvas.draw_rect(map[i as usize]);
        if res.is_err() {
            print!("Error when drawing rectangle {}", i);
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
            print!("Error when drawing rectangle at pos {} {}", pos.x, pos.y);
        }
    }
}

pub fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
          game_data: &GameData,
          i: u8) {
    render_map(canvas, &game_data.map, i);
    draw_snake(canvas, &game_data.map, &game_data.snake);
    canvas.present();
}