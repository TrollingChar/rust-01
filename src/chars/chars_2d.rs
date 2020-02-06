use crate::chars::char::{Char};
use sdl2::render::{WindowCanvas, Texture};


pub struct Chars2D {
    w: usize,
    h: usize,
    chars: Vec<Char>,
}


impl Chars2D {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, chars: vec![Char::new(0); w * h] }
    }

    pub fn with_filler(w: usize, h: usize, filler: Char) -> Self {
        Self { w, h, chars: vec![filler; w * h] }
    }

    pub fn get(&self, x: usize, y: usize) -> Char {
        self.chars[self.i(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Char {
        let i = self.i(x, y);
        &mut self.chars[i]
    }

    pub fn set(&mut self, x: usize, y: usize, c: Char) {
        let i = self.i(x, y);
        self.chars[i] = c;
    }

    pub fn print(&mut self, x: usize, y: usize, c: Char) {
        self.get_mut(x, y).print(c);
    }

    pub fn render(&self, canvas: &mut WindowCanvas, font: &mut Texture) {
        for x in 0..self.w {
            for y in 0..self.h {
                self.get(x, y).render(canvas, font, x, y);
            }
        }
    }

    fn i(&self, x: usize, y: usize) -> usize { x + y * self.w }
}