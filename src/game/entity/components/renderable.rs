use super::Renderable;
use rltk::RGB;

type Colour = (u8, u8, u8);

#[derive(PartialEq, Debug)]
pub enum GlyphType {
    Static,
    Random,
    Animated,
}

impl Renderable {
    pub fn new(glyphs: Vec<char>, kind: GlyphType, fg: Colour, bg: Colour) -> Self {
        Self {
            g_id: Renderable::set_g_id(&kind, glyphs.len()),
            kind: kind,
            glyph: Renderable::convert_glyphs_to_u8(glyphs),
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }

    // use v:Into<Option<usize>> for dynamic typing of input value
    pub fn g(&self) -> &u8 {
        match &self.kind {
            GlyphType::Static => &self.glyph[self.g_id],
            GlyphType::Random => &self.glyph[self.g_id],
            // TODO: create animation mechanism
            GlyphType::Animated => &self.glyph[self.g_id as usize],
        }
    }

    fn convert_glyphs_to_u8(glyphs: Vec<char>) -> Vec<u8> {
        glyphs.iter().map(|glyph| rltk::to_cp437(*glyph)).collect()
    }

    fn set_g_id(kind: &GlyphType, glyph_count: usize) -> usize {
        match kind {
            GlyphType::Static => 0 as usize,
            GlyphType::Animated => 0 as usize,
            GlyphType::Random => rltk::RandomNumberGenerator::new().range(0, glyph_count) as usize,
        }
    }
}
