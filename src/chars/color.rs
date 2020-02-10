use sdl2::render::{Texture, WindowCanvas};


#[derive(Copy, Clone)]
pub enum Color {
    Transparent,
    Indexed(u8),
    Rgb(u8, u8, u8),
}


impl Color {
    pub fn affect_texture(self, texture: &mut Texture) {
        match self {
            Color::Transparent => {}
            Color::Indexed(_) => {}
            Color::Rgb(r, g, b) => texture.set_color_mod(r, g, b),
        }
    }

    pub fn affect_canvas(self, canvas: &mut WindowCanvas) {
        match self {
            Color::Transparent => canvas.set_draw_color((0, 0, 0)),
            Color::Indexed(_) => {}
            Color::Rgb(r, g, b) => canvas.set_draw_color((r, g, b)),
        }
    }
}