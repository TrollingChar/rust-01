use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use crate::chars::chars_2d::Chars2D;
use crate::chars::char::Char;


pub struct Game {
    over: bool,
    dirty: bool,
    time: i64,
//    matrix: Chars2D,
}


impl Game {
    pub fn new() -> Self {
        Game {
            over: false,
            dirty: true,
            time: 0,
//            matrix: Chars2D::new(40, 30, Char::new(0)),
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
//        self.matrix.render(canvas, font);
        let mut matrix = Chars2D::new(40, 30);
        let mut t = self.time;
        let mut x = 38;
        while t != 0 {
            matrix.set(x, 1, Char::new('0' as u8 + (t % 10) as u8));
            t /= 10;
            x -= 1;
        }
        matrix.render(canvas, font);
        canvas.present();
        self.dirty = false;
    }
    pub fn over(&self) -> bool { self.over }
    pub fn dirty(&self) -> bool { self.dirty }
}