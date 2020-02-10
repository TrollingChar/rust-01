use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::chars::{Color, Glyph};
use crate::r#const::*;
use sdl2::pixels;


#[derive(Copy, Clone)]
pub struct Char {
    glyph: Glyph,
    color: Color,
    background: Color,
}


impl Char {
    pub fn null() -> Self {
        Self {
            glyph: Glyph::Transparent,
            color: Color::Transparent,
            background: Color::Transparent,
        }
    }

    pub fn new(glyph: Glyph) -> Self {
        Self { glyph, color: Color::Rgb(0xff, 0xff, 0xff), background: Color::Transparent }
    }

    pub fn with_color(glyph: Glyph, color: Color) -> Self {
        Self { glyph, color, background: Color::Transparent }
    }

    pub fn with_colors(glyph: Glyph, color: Color, background: Color) -> Self {
        Self { glyph, color, background }
    }

    pub fn render_background(self, x: u32, y: u32, canvas: &mut WindowCanvas) {
        self.background.affect_canvas(canvas);
        let _ = canvas.fill_rect(
            Rect::new(
                (x * CHAR_DISPLAYED_W) as i32,
                (y * CHAR_DISPLAYED_H) as i32,
                CHAR_DISPLAYED_W,
                CHAR_DISPLAYED_H,
            )
        );
    }

    pub fn render_glyph(self, x: u32, y: u32, canvas: &mut WindowCanvas, font: &mut Texture) {
        self.color.affect_texture(font);
        let _ = canvas.copy(
            font,
            self.glyph.src_rect(),
            Rect::new(
                (x * CHAR_DISPLAYED_W) as i32,
                (y * CHAR_DISPLAYED_H) as i32,
                CHAR_DISPLAYED_W,
                CHAR_DISPLAYED_H,
            ),
        );
    }
}