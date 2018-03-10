extern crate sfl_parser;

use sfl_parser::BMFont;

fn main() {
    let bmfont = match BMFont::load_and_parse("examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);
}
