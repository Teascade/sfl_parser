use super::setup;

#[test]
fn test_character_id() {
    let bmfont = setup();
    assert_eq!(bmfont.chars[&65].id, 65);
}

#[test]
fn test_character_coordinates() {
    let bmfont = setup();
    assert_eq!(bmfont.chars[&65].x, 77);
    assert_eq!(bmfont.chars[&65].y, 168);
}

#[test]
fn test_character_size() {
    let bmfont = setup();
    assert_eq!(bmfont.chars[&65].width, 18);
    assert_eq!(bmfont.chars[&65].height, 33);
}

#[test]
fn test_character_offsets() {
    let bmfont = setup();
    assert_eq!(bmfont.chars[&65].xoffset, 2);
    assert_eq!(bmfont.chars[&65].yoffset, 23);
}

#[test]
fn test_character_xadvance() {
    let bmfont = setup();
    assert_eq!(bmfont.chars[&65].xadvance, 22);
}
