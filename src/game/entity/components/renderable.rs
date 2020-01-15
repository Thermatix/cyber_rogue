use super::Renderable;
use rltk::RGB;

type Colour = (u8, u8, u8);

impl Renderable {
    pub fn new(glyph: u8, fg: Colour, bg: Colour) -> Self {
        Self {
            glyph: glyph,
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }
}
