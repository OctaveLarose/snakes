use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
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

    canvas.present();
}