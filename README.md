# aseprite

A crate for loading data from the [aseprite](https://www.aseprite.org/) sprite editor.  Should go along well with the [tiled](https://github.com/mattyhall/rs-tiled) crate, I hope!

It does not load any actual images, just the metadata.  Currently it only loads aseprite's JSON export format.

Automatically exporting a sprite to a given format is documented here: <https://www.aseprite.org/docs/cli/>

# Docs

Documentation for the latest version is on [docs.rs](https://docs.rs/aseprite/)

# Example

Export sprite sheet with:

```
aseprite -b boonga.ase --sheet boonga.png --format json-array --list-tags --list-layers --data boonga.json
```

Then write a program to load it:

```rust
extern crate serde_json;
extern crate aseprite;

use std::fs::File;

fn main() {
   let file = File::open("boonga.json").unwrap();
   let spritesheet: aseprite::SpritesheetData = serde_json::from_reader(file).unwrap();
   println!("Spritesheet is {:?}", spritesheet);
}
```
