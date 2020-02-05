use std::ptr::null_mut;

use sdl2::event::{Event, EventType};
use sdl2::image::LoadTexture;

use crate::game::Game;


mod game;
mod chars;


fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;

    let video = sdl.video()?;
    let events = sdl.event()?;
    let mut event_pump = sdl.event_pump()?;
    let timers = sdl.timer()?;

    let window = video.
        window("Trollingchars 2020", 800, 600).
        position_centered().
        build().
        expect("Could not make a window");
    let window_id = window.id();

    let mut canvas = window.
        into_canvas().
        build().
        expect("Could not make a canvas");
    canvas.set_draw_color((0x00, 0x33, 0x33));

    let texture_creator = canvas.texture_creator();
    let mut font_texture = texture_creator.load_texture("assets/chars.png")?;

    let mut game = Game::new();
    game.render(&mut canvas, &mut font_texture);

    let _timer = timers.add_timer(10, Box::new(|| {
        let _result = events.push_event(Event::User {
            timestamp: 0,
            window_id,
            type_: EventType::User as u32,
            code: 0,
            data1: null_mut(),
            data2: null_mut(),
        });
        10
    }));

    for event in event_pump.wait_iter() {
        println!("{:?}", &event);
        game.on_event(&event);
        if game.over() { break; }
        if game.dirty() {
            game.render(&mut canvas, &mut font_texture);
        }
    }

    Ok(())
}
/*
**  Что дальше: надо сделать игрока и чанки с генерацией их из шума (двухмерные)
*/