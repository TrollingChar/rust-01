use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};

use crate::chars::{Chars2D, Char, Glyph, Color};
use crate::r#const::*;


pub struct Game {
    over: bool,
    dirty: bool,
    time: i64,
}


impl Game {
    pub fn new() -> Self {
        Game {
            over: false,
            dirty: true,
            time: 0,
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
        canvas.set_draw_color((0, 0, 0));
        canvas.clear();
        let mut matrix = Chars2D::new(GAME_W, GAME_H);
        matrix.set_char(1, 1, Char::with_colors(Glyph::Printable(168), Color::Rgb(0, 255, 0), Color::Rgb(255, 0, 0)));
        matrix.set_char(2, 2, Char::with_colors(Glyph::Printable(169), Color::Rgb(255, 0, 0), Color::Rgb(0, 255, 0)));
        matrix.render(canvas, font);
        canvas.present();
        self.dirty = false;
    }

    pub fn over(&self) -> bool { self.over }
    pub fn dirty(&self) -> bool { self.dirty }
}