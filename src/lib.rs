//! A lightweight easy-to-use bitmap font  loader and parser for .sfl files.
//!
//! # Examples
//! ```
//! use sfl_parser::BMFont;
//!
//! let bmfont = match BMFont::load_and_parse("examples/fonts/iosevka.sfl") {
//!     Ok(bmfont) => bmfont,
//!     Err(_) => panic!("Failed to load iosevka.sfl"),
//! };
//!
//! println!("bmfont: {}", bmfont);
//! ```

#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;

/// Represents a single character in the bitmap font atlas. Contains coordinates, sizes, offsets and advances (everything required to render letters from the atlas).
#[derive(Debug)]
pub struct BMCharacter {
    /// char id of the character.
    pub id: u32,
    /// x-position of the character on the atlas.
    pub x: u32,
    /// y-position of the character on the atlas.
    pub y: u32,
    /// Width of the character on the atlas.
    pub width: u32,
    /// Height of the character.
    pub height: u32,
    /// x-offset of the character.
    pub xoffset: u32,
    /// y-offset of the character.
    pub yoffset: u32,
    /// x-advance of the character.
    pub xadvance: u32,
}

/// Loaded and parsed struct of an .sfl file (a bitmap font file).
#[derive(Debug)]
pub struct BMFont {
    /// The name of the font.
    pub font_name: String,
    /// The path of the image atlas for the font.
    pub image_path: PathBuf,
    /// Hashmap of the characters in the font. &ltCharID, [`BMCharacter`][bmcharacter]&gt
    ///
    /// [bmcharacter]: struct.BMCharacter.html
    pub chars: HashMap<u32, BMCharacter>,
    /// Line height of the font.
    pub line_height: u32,
    /// Size of the font.
    pub size: u32,
}

impl BMFont {
    /// Load and parse a `BMFont` from the given `path`, which should be an .sfl file.
    ///
    /// # Examples
    /// ```
    /// use sfl_parser::BMFont;
    ///
    /// let bmfont = match BMFont::load_and_parse("examples/fonts/iosevka.sfl") {
    ///     Ok(bmfont) => bmfont,
    ///     Err(_) => panic!("Failed to load iosevka.sfl"),
    /// };
    ///
    /// println!("bmfont: {}", bmfont);
    /// ```
    pub fn load_and_parse<T: Into<PathBuf>>(path: T) -> Result<BMFont, String> {
        let path = path.into();
        let mut file;
        match File::open(&path) {
            Ok(f) => file = f,
            Err(error) => return Err(format!("Error while loading .sfl file: {}", error)),
        }

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let mut lines = buffer.lines();

        if lines.clone().count() < 5 {
            return Err("Erronous .sfl file; too few lines to initialize.".to_owned());
        }

        // Take font name from first line
        let font_name = lines.next().unwrap().to_owned();
        // Take line height and font size from second line
        let line_h_and_size = lines.next().unwrap().to_owned();
        let mut parts = line_h_and_size.split(' ');
        let size;
        let line_height;
        if parts.clone().count() == 2 {
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => size = number,
                Err(error) => return Err(format!("Error parsing line height: '{}'", error)),
            }
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => line_height = number,
                Err(error) => return Err(format!("Error parsing size: '{}'", error)),
            }
        } else {
            return Err(format!(
                "Second line does not contain two values formatted as 'line-height size'"
            ));
        }

        // Read image path
        let image_name = lines.next().unwrap().to_owned();
        let mut image_path;
        if let Some(path) = path.parent() {
            image_path = path.to_path_buf();
            image_path.push(image_name);
        } else {
            return Err(format!("Unable to retrieve path parent."));
        }

        // Read characters
        let character_amount;
        match lines.next().unwrap().to_owned().parse::<u32>() {
            Ok(amount) => character_amount = amount,
            Err(_) => return Err(format!("Error while parsing character amount at line: 4")),
        }

        if lines.clone().count() + 5 < 5 + character_amount as usize {
            return Err(format!("Erronous .sfl file; character amount (line 4) does not match actual character amount; is {}, should be {}", lines.count() + 5, 5 + character_amount));
        }

        let mut chars = HashMap::<u32, BMCharacter>::new();
        for i in 0..character_amount {
            let character = BMFont::read_character(lines.next().unwrap().to_owned(), i + 1);
            match character {
                Ok(ch) => chars.insert(ch.id, ch),
                Err(error) => return Err(error),
            };
        }

        return Ok(BMFont {
            font_name,
            image_path,
            chars,
            line_height,
            size,
        });
    }

    fn read_character(line: String, line_number: u32) -> Result<BMCharacter, String> {
        let mut parts = line.split(' ');
        if parts.clone().count() < 8 {
            return Err(format!(
                "Too few parts in character at line: {}",
                line_number
            ));
        }

        let mut numbers: Vec<u32> = vec![0; 8];
        for i in 0..8 {
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => numbers[i] = number,
                Err(_) => {
                    return Err(format!(
                        "Error while parsing number at line: {}",
                        line_number
                    ));
                }
            }
        }

        Ok(BMCharacter {
            id: numbers[0],
            x: numbers[1],
            y: numbers[2],
            width: numbers[3],
            height: numbers[4],
            xoffset: numbers[5],
            yoffset: numbers[6],
            xadvance: numbers[7],
        })
    }
}

impl Display for BMFont {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        write!(
            f,
            "BMFont: {{ name: {}, image_path: {:?}, line_height: {}, size: {}, amount of characters: {} }}",
            self.font_name,
            self.image_path,
            self.line_height,
            self.size,
            self.chars.len()
        )
    }
}
