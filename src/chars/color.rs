use sdl2::render::Texture;


#[derive(Copy, Clone)]
pub enum Color {
    Transparent,
    Indexed(u8),
    Rgb(u8, u8, u8),
}


impl Color {
    pub fn affect_texture(self, texture: &mut Texture) {
        match self {
            Color::Transparent => {},
            Color::Indexed(_) => {}
            Color::Rgb(r, g, b) => texture.set_color_mod(r, g, b),
        }
    }
}