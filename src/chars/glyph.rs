use sdl2::rect::Rect;


#[derive(Copy, Clone)]
pub enum Glyph {
    Transparent,
    Printable(u8),
}


impl Glyph {
    pub fn src_rect(self) -> Rect {
        match self {
            Glyph::Transparent => Rect::new(0, 0, 10, 10),
            Glyph::Printable(i) => Rect::new((i as i32 & 0xf) * 10, (i as i32 >> 4) * 10, 10, 10),
        }
    }
}