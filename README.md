# sfl_parser

[![Build Status](https://travis-ci.org/Teascade/sfl_parser.svg?branch=1.3.1)](https://travis-ci.org/Teascade/sfl_parser)
[![Docs](https://docs.rs/sfl_parser/badge.svg)](https://docs.rs/sfl_parser)
[![Crates.io](https://img.shields.io/crates/v/sfl_parser.svg)](https://crates.io/crates/sfl_parser)

## sfl_parser has permanently moved to [`bmfont_parser`](https://github.com/teascade/bmfont_parser), as the name has grown inaccurate to the functionality of this crate

A lightweight and easy-to-use .sfl file (bitmap font) parser made with Rust.

### How to use
Documentation at [docs.rs][docs] or simply:

1. Add the following to your dependencies:  
   ```toml
   [dependencies]
   sfl_parser="1.3"
   ```
2. To your Rust project add the following line:
   ```rust
   extern crate sfl_parser;
   ```
3. You're done! Here is an example of how to use it:
   ```rust
    use sfl_parser::BMFont;

    let bmfont = match BMFont::from_path("examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);

    // Or Alternatively

    let iosevka_sfl = include_str!("examples/fonts/iosevka.sfl");

    let bmfont = match BMFont::from_loaded(iosevka_sfl, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);
   ```

### License
This crate is distributed under the terms of [the MIT License][license].

[license]: LICENSE.md
[docs]: https://docs.rs/sfl_parser
