use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;


#[derive(Copy, Clone)]
pub struct CharCode(pub u8);


impl CharCode {
    fn x(&self) -> u8 { self.0 & 0xf }
    fn y(&self) -> u8 { self.0 >> 4 }
}


#[derive(Copy, Clone)]
pub struct Char {
    pub code: CharCode,
//    color: CharColor,
}


impl Char {
    fn render (self, canvas: &mut WindowCanvas, font: &mut Texture, x: usize, y: usize) {
        font.set_color_mod(0x99, 0xff, 0x99);
        let _result = canvas.copy(
            font,
            Rect::new((self.code.x() * 10) as i32, (self.code.y() * 10) as i32, 10, 10),
            Rect::new((x * 20) as i32, (y * 20) as i32, 20, 20),
        );
    }
}


pub struct Chars2D {
    w: usize,
    h: usize,
    chars: Vec<Char>,
}


impl Chars2D {
    pub fn new(w: usize, h: usize, filler: Char) -> Self { Self { w, h, chars: vec![filler; w * h] } }
    pub fn get(&self, x: usize, y: usize) -> Char { self.chars[self.i(x, y)] }
    pub fn set(&mut self, x: usize, y: usize, c: Char) {
        let i = self.i(x, y);
        self.chars[i] = c;
    }
    pub fn print(&mut self, x: usize, y: usize, c: Char) { unimplemented!() }
    pub fn render(&self, canvas: &mut WindowCanvas, font: &mut Texture) {
        for x in 0..self.w {
            for y in 0..self.h {
                self.get(x, y).render(canvas, font, x, y);
            }
        }
    }
    fn i(&self, x: usize, y: usize) -> usize { x + y * self.w }
}