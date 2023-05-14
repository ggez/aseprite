# aseprite

A Rust crate for loading data from the [aseprite](https://www.aseprite.org/) sprite editor 

<a href="https://github.com/ggez/aseprite/actions"><img src="https://github.com/ggez/aseprite/workflows/CI/badge.svg" alt="CI"/></a>
[![Cargo](https://img.shields.io/crates/v/aseprite.svg)](https://crates.io/crates/aseprite) [![Downloads](https://img.shields.io/crates/d/aseprite.svg)](#downloads)

Should go along well with the [tiled](https://github.com/mattyhall/rs-tiled) crate, I hope! It does not load any actual images, just the metadata. Currently it only loads aseprite's JSON export format.

Automatically exporting a sprite to a given format is documented here: <https://www.aseprite.org/docs/cli/>

# Docs

Documentation for the latest version is on [docs.rs](https://docs.rs/aseprite/).

# Example

Export sprite sheet with:

```sh
aseprite -b boonga.ase --sheet boonga.png --format json-array --list-tags --list-layers --data boonga.json
```

Then write a program to load it:

```rust
use aseprite::SpritesheetData;
use std::fs::File;

fn main() {
   let file = File::open("boonga.json").unwrap();
   let spritesheet: SpritesheetData = serde_json::from_reader(file).unwrap();
   println!("Spritesheet is {:?}", spritesheet);
}
```
