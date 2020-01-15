use super::Renderable;
use rltk::RGB;

type Colour = (u8, u8, u8);

impl Renderable {
    pub fn new(glyph: char, fg: Colour, bg: Colour) -> Self {
        Self {
            glyph: rltk::to_cp437(glyph),
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }
}
