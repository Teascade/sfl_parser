use super::{from_loaded_setup, from_path_setup};
use std::path::PathBuf;

#[test]
fn test_font_name() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.font_name, "Iosevka");
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.font_name, "Iosevka");
}

#[test]
fn test_line_height() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.line_height, 56);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.line_height, 56);
}

#[test]
fn test_size() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.size, 32);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.size, 32);
}

#[test]
fn test_image_path() {
    let bmfont = from_path_setup();
    assert_eq!(
        bmfont.image_path,
        PathBuf::from("examples/fonts/iosevka.png")
    );
    let bmfont = from_loaded_setup();
    assert_eq!(
        bmfont.image_path,
        PathBuf::from("examples/fonts/iosevka.png")
    );
}

#[test]
fn test_character_amount() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars.len(), 191);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars.len(), 191);
}
