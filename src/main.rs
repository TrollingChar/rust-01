use sdl2::event::{Event, EventType};
use sdl2::image::LoadTexture;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;


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

    let texture_creator = canvas.texture_creator();
    let mut font_texture = texture_creator.load_texture("assets/chars.png")?;

    canvas.set_draw_color((0x00, 0x33, 0x33));
    render(&mut canvas, &mut font_texture);

    let _timer = timers.add_timer(1000, Box::new(|| {
        let _result = events.push_event(Event::User {
            timestamp: 0,
            window_id,
            type_: EventType::User as u32,
            code: 0,
            data1: std::ptr::null_mut(),
            data2: std::ptr::null_mut(),
        });
        1000
    }));

    for event in event_pump.wait_iter() {
        println!("{:?}", event);
        match event {
            Event::Quit { .. } => break,
            Event::User { .. } => render(&mut canvas, &mut font_texture),
            _ => {}
        }
    }

    Ok(())
}

fn render(canvas: &mut WindowCanvas, font_texture: &mut Texture) -> () {
    canvas.clear();
    font_texture.set_color_mod(0x99, 0xff, 0x99);
    canvas.copy(font_texture, Rect::new(0, 30, 10, 10), Rect::new(20, 20, 20, 20));
    canvas.present();
}