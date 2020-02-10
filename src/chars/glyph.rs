use sdl2::rect::Rect;

use crate::r#const::*;


#[derive(Copy, Clone)]
pub enum Glyph {
    Transparent,
    Printable(u8),
}


impl Glyph {
    pub fn src_rect(self) -> Rect {
        match self {
            Glyph::Transparent => Rect::new(0, 0, CHAR_STORED_W, CHAR_STORED_H),
            Glyph::Printable(i) => Rect::new(
                (i as i32 & 0xf) * CHAR_STORED_W as i32,
                (i as i32 >> 4) * CHAR_STORED_H as i32,
                CHAR_STORED_W,
                CHAR_STORED_H
            ),
        }
    }
}