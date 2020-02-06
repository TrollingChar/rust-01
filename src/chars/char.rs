use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;


#[derive(Copy, Clone)]
pub struct Char {
    code: Code,
}


impl Char {
    pub fn new<T: Into<Code>>(code: T) -> Self {
        Char { code: code.into() }
    }

    pub fn print(&mut self, c: Self) {
        if c.code() != Code(0) { self.code = c.code; }
    }

    pub(super) fn render(self, canvas: &mut WindowCanvas, font: &mut Texture, x: usize, y: usize) {
        font.set_color_mod(0x99, 0xff, 0x99);
        let _result = canvas.copy(
            font,
            Rect::new((self.code.x() * 10) as i32, (self.code.y() * 10) as i32, 10, 10),
            Rect::new((x * 20) as i32, (y * 20) as i32, 20, 20),
        );
    }

    pub fn code(&self) -> Code {
        self.code
    }

    pub fn set_code<T: Into<Code>>(&mut self, code: T) {
        self.code = code.into();
    }
}


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Code(pub u8);


impl Code {
    pub(super) fn x(&self) -> u8 { self.0 & 0xf }
    pub(super) fn y(&self) -> u8 { self.0 >> 4 }
}


impl From<char> for Code {
    fn from(c: char) -> Self {
        Self(c as u8)
    }
}


impl From<u8> for Code {
    fn from(n: u8) -> Self {
        Self(n)
    }
}


pub enum Color {
    None,
    Rgb { r: u8, g: u8, b: u8 },
}


impl Color {

}