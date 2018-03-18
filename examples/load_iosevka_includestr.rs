extern crate sfl_parser;

use sfl_parser::BMFont;

static IOSEVKA_SFL: &'static str = include_str!("fonts/iosevka.sfl");

fn main() {
    let bmfont = match BMFont::from_loaded(IOSEVKA_SFL, "examples/fonts/iosevka.png") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);
}
