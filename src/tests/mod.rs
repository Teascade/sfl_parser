mod bmfont;
mod bmcharacter;

use super::BMFont;

pub fn setup() -> BMFont {
    match BMFont::from_path("examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }
}
