mod bmfont;
mod bmcharacter;

use super::BMFont;

pub fn from_path_setup() -> BMFont {
    match BMFont::from_path("examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }
}

pub fn from_loaded_setup() -> BMFont {
    let iosevka_sfl = include_str!("../../examples/fonts/iosevka.sfl");
    match BMFont::from_loaded(iosevka_sfl, "examples/fonts/iosevka.png") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }

}