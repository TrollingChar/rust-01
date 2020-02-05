use sdl2::event::Event;
use sdl2::render::{WindowCanvas, Texture};
use crate::chars::{Chars2D, CharCode, Char};


pub struct Game {
    over: bool,
    dirty: bool,
    time: i64,
    matrix: Chars2D,
}


impl Game {
    pub fn new() -> Self {
        Game {
            over: false,
            dirty: true,
            time: 0,
            matrix: Chars2D::new(40, 30, Char { code: CharCode('+' as u8) }),
        }
    }
    pub fn on_event(&mut self, event: &Event) {
        match event {
            Event::Quit { .. } => self.over = true,
            Event::User { .. } => {
                self.time += 1;
                self.dirty = true;
            }
            _ => {}
        }
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas, font: &mut Texture) {
        canvas.clear();
        self.matrix.render(canvas, font);
        canvas.present();
        self.dirty = false;
    }
    pub fn over(&self) -> bool { self.over }
    pub fn dirty(&self) -> bool { self.dirty }
}