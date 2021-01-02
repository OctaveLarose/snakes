use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum UpdateValue { Ok, GameStop }

pub fn update(event_pump: &mut sdl2::EventPump, i: &mut u8) -> UpdateValue {
    *i = (*i + 1) % 255;

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
                print!("Key pressed\n");
                println!("{:?}", event);
            },
            _ => {}
        }
    }

    UpdateValue::Ok
}