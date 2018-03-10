mod bmfont;
mod bmcharacter;

use super::BMFont;

pub fn setup() -> BMFont {
    match BMFont::load_and_parse("examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }
}
