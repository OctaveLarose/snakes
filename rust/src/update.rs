use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::{Snake, Direction, Pos, GameData};

pub enum UpdateValue { Ok, GameStop }

pub fn spawn_food(_nbr: u8) -> [Pos; crate::MAX_FOOD as usize] {
    let food_pos: [Pos; crate::MAX_FOOD as usize] = [Pos{x: 0, y: 0}; crate::MAX_FOOD as usize];

    food_pos
}

fn update_snake_pos(snake: &mut Snake) {
    let mut last_pos: Pos = snake.body[0];

    match snake.dir {
        Direction::NORTH => snake.body[0].y = snake.body[0].y - 1,
        Direction::EAST => snake.body[0].x = snake.body[0].x + 1,
        Direction::SOUTH => snake.body[0].y = snake.body[0].y + 1,
        Direction::WEST => snake.body[0].x = snake.body[0].x - 1,
    }

    for (idx, pos) in snake.body.iter_mut().enumerate() {
        if idx == 0 {
            continue
        }

        let tmp_pos = pos.clone();
        pos.x = last_pos.x;
        pos.y = last_pos.y;
        last_pos = tmp_pos;
    }
}

fn change_snake_dir(event: sdl2::event::Event, snake: &mut Snake) {
    snake.dir = match event {
        sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Up), .. } => Direction::NORTH,
        sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Right), .. } => Direction::EAST,
        sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Down), .. } => Direction::SOUTH,
        sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Left), .. } => Direction::WEST,
        _ => snake.dir
    }
}

pub fn update(event_pump: &mut sdl2::EventPump, game_data: &mut GameData, i: &mut u8) -> UpdateValue {
    *i = (*i + 1) % 255;
    update_snake_pos(&mut game_data.snake);

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return UpdateValue::GameStop;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } |
            Event::KeyDown { keycode: Some(Keycode::Down), .. } |
            Event::KeyDown { keycode: Some(Keycode::Left), .. } |
            Event::KeyDown { keycode: Some(Keycode::Right), .. }  => {
                change_snake_dir(event, &mut game_data.snake);
            },
            _ => {}
        }
    }

    UpdateValue::Ok
}