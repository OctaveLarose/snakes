use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;

use crate::{Snake, Direction, Pos, GameData, MAP_X, MAP_Y};

pub enum UpdateValue { Ok, GameLost, GameStop }

pub fn spawn_food() -> Pos
{
    let mut rng = rand::thread_rng();

    Pos{x: rng.gen_range(0..MAP_X), y: rng.gen_range(0..MAP_Y)}
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

fn check_snake_pos(snake: &mut Snake, mut food_pos: &mut Pos) -> UpdateValue {
    // Checking if the snake is out of bounds
    match snake.body[0] {
        pos if pos.x < 0 => UpdateValue::GameLost,
        pos if pos.x >= MAP_X => UpdateValue::GameLost,
        pos if pos.y < 0 => UpdateValue::GameLost,
        pos if pos.y >= MAP_Y => UpdateValue::GameLost,
        _ => UpdateValue::Ok
    };

    // Checking collision with body parts
    for (idx, body_part) in snake.body.iter().enumerate() {
        if idx == 0 {
            continue
        }

        if snake.body[0] == *body_part {
            return UpdateValue::GameLost;
        }
    }

    // Checking collision with food
    if snake.body[0] == *food_pos {
        let new_food = spawn_food();
        food_pos.x = new_food.x;
        food_pos.y = new_food.y;
    }
    UpdateValue::Ok
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

pub fn update(event_pump: &mut sdl2::EventPump,
              game_data: &mut GameData,
              map_blueshift: &mut u8) -> UpdateValue
{
    *map_blueshift = (*map_blueshift + 1) % 255;
    update_snake_pos(&mut game_data.snake);

    if let UpdateValue::GameLost = check_snake_pos(&mut game_data.snake, &mut game_data.food_position) {
        return UpdateValue::GameLost;
}   ;

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