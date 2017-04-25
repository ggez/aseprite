# aseprite

A crate for loading data from the [aseprite](https://www.aseprite.org/) sprite editor.  Should go along well with the [tiled](https://github.com/mattyhall/rs-tiled) crate, I hope!

It does not load any actual images, just the metadata.  Currently it only loads aseprite's JSON export format, and only when exported in "array" mode.  I've yet to find a use case that won't cover.

Automatically exporting a sprite to a given format is documented here: <https://www.aseprite.org/docs/cli/>

# Example

```rust
extern crate serde_json;
extern crate aseprite;

fn main() {
   let file = File::open("assets/boonga.json");
   let spritesheet: aseprite::SpritesheetData = serde_json::from_reader(file).unwrap();
   println!("Spritesheet is {:?}", spritesheet);
}
```