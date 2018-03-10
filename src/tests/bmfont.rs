use super::setup;
use std::path::PathBuf;

#[test]
fn test_font_name() {
    let bmfont = setup();
    assert_eq!(bmfont.font_name, "Iosevka");
}

#[test]
fn test_line_height() {
    let bmfont = setup();
    assert_eq!(bmfont.line_height, 56);
}

#[test]
fn test_size() {
    let bmfont = setup();
    assert_eq!(bmfont.size, 32);
}

#[test]
fn test_image_path() {
    let bmfont = setup();
    assert_eq!(
        bmfont.image_path,
        PathBuf::from("examples/fonts/iosevka.png")
    );
}

#[test]
fn test_character_amount() {
    let bmfont = setup();
    assert_eq!(bmfont.chars.len(), 191);
}
