sfl_parser
==========
A lightweight and easy-to-use .sfl file (bitmap font) parser made with Rust.

### How to use

1. Add the following to your dependencies:  
   ```toml
   [dependencies]
   sfl_parser="1.0"
   ```
2. To your Rust project add the following line:
   ```rust
   extern crate sfl_parser;
   ```
3. You're done! Here is an example of how to use it:
   ```rust
   use sfl_parser::BMFont;

   let bmfont = match BMFont::load_and_parse("examples/fonts/iosevka.sfl") {
       Ok(bmfont) => bmfont,
       Err(_) => panic!("Failed to load iosevka.sfl"),
   };

   println!("bmfont: {}", bmfont);
   ```

### License
This crate is distributed under the terms of [the MIT License][license].

[license]: LICENSE.md
