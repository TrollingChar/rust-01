use crate::chars::char::Char;
use sdl2::render::{WindowCanvas, Texture};


pub struct Chars2D {
    w: usize,
    h: usize,
    chars: Vec<Char>, // TODO: перейти на RawVec когда он станет стабильным
}


impl Chars2D {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, chars: vec![Char::null(); w * h] }
    }

    pub fn char(&self, x: usize, y: usize) -> Char {
        self.chars[x + y * self.w]
    }

    pub fn char_mut(&mut self, x: usize, y: usize) -> &mut Char {
        &mut self.chars[x + y * self.w]
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: Char) {
        self.chars[x + y * self.w] = c
    }

    pub fn render(&self, canvas: &mut WindowCanvas, font: &mut Texture) {
        for x in 0..self.w {
            for y in 0..self.h {
                self.char(x, y).render_background(x as u32, y as u32, canvas);
            }
        }
        for x in 0..self.w {
            for y in 0..self.h {
                self.char(x, y).render_glyph(x as u32, y as u32, canvas, font);
            }
        }
    }
}